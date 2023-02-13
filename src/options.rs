use miette::IntoDiagnostic;
use watchexec::{
    command::Command,
    config::{InitConfig, RuntimeConfig},
    handler::PrintDebug,
    Watchexec,
};

pub async fn get_options() {
    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));
    let mut runtime = RuntimeConfig::default();

    runtime.pathset(["src"]);

    runtime.command(Command::Exec {
        prog: "solana".to_string(),
        args: vec!["--version".to_string()],
    });

    let w = Watchexec::new(init, runtime.clone())?;

    w.main();
}
