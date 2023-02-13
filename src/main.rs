use std::time::Duration;

use miette::{IntoDiagnostic, Result};
use watchexec::{
    action::{Action, Outcome},
    command::Command,
    config::{InitConfig, RuntimeConfig},
    error::ReconfigError,
    event::Event,
    fs::Watcher,
    signal::source::MainSignal,
    ErrorHook, Watchexec,
};

// Run with: `env RUST_LOG=debug cargo run --example print_out`
#[tokio::main]
async fn main() -> Result<()> {
    let mut init = InitConfig::default();
    init.on_error(|err: ErrorHook| async move {
        eprintln!("Watchexec Runtime Error: {}", err.error);
        Ok::<(), std::convert::Infallible>(())
    });

    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["src"]);
    runtime.command(Command::Exec {
        prog: "solana".to_string(),
        args: vec!["--version".to_string()],
    });

    let wx = Watchexec::new(init, runtime.clone())?;
    let w = wx.clone();

    let config = runtime.clone();
    runtime.on_action(move |action: Action| {
        let mut config = config.clone();
        let w = w.clone();
        async move {
            eprintln!("Watchexec Action: {action:?}");

            let sigs = action
                .events
                .iter()
                .flat_map(Event::signals)
                .collect::<Vec<_>>();

            if sigs.iter().any(|sig| sig == &MainSignal::Interrupt) {
                action.outcome(Outcome::Exit);
            } else if sigs.iter().any(|sig| sig == &MainSignal::User1) {
                eprintln!("Switching to native for funsies");
                config.file_watcher(Watcher::Native);
                w.reconfigure(config)?;
            } else if sigs.iter().any(|sig| sig == &MainSignal::User2) {
                eprintln!("Switching to polling for funsies");
                config.file_watcher(Watcher::Poll(Duration::from_millis(50)));
                w.reconfigure(config)?;
            } else {
                action.outcome(Outcome::if_running(
                    Outcome::both(Outcome::Stop, Outcome::Start),
                    Outcome::Stop,
                ));
                // w.reconfigure(config)?;
            }

            Ok::<(), ReconfigError>(())
        }
    });

    wx.reconfigure(runtime)?;
    wx.main().await.into_diagnostic()??;

    Ok(())
}
