use clap::Parser;
use miette::{IntoDiagnostic, Result};
// use std::process::Command;

use std::time::Duration;
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
// fn main() {
//     // Command::new("solana")
//     //     .args(["config", "set", "--url", "http://api.devnet.solana.com"])
//     //     .output()
//     //     .expect("solana  commad failed");
//     // let output = Command::new("solana-test-validator")
//     //     .output()
//     //     .expect("solana  commad failed");

//     // println!(
//     //     "started the server {}",
//     //     String::from_utf8_lossy(&output.stdout)
//     // );

//     let mut init = InitConfig::default();
// }

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();

    let mut init = InitConfig::default();
    init.on_error(|err: ErrorHook| async move {
        eprintln!("Watchexec Runtime Error: {}", err.error);
        Ok::<(), std::convert::Infallible>(())
    });

    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["src"]);
    runtime.command(Command::Exec {
        prog: "date".into(),
        args: Vec::new(),
    });
    let wx = Watchexec::new(init, runtime.clone())?;
    let w = wx.clone();

    let config = runtime.clone();
    runtime.on_action(move |action: Action| {
        let mut config = config.clone();
        let w = w.clone();
        async move {
            // eprintln!("Watchexec Action: {action:?}");

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
                    Outcome::Start,
                ));
            }

            Ok::<(), ReconfigError>(())
        }
    });

    wx.reconfigure(runtime)?;
    wx.main().await.into_diagnostic()??;

    Ok(())
}
