// use clap::{App, AppSettings, Arg, ArgMatches, ErrorKind, SubCommand};
// use std::{env, process};

// pub fn parse() -> ArgMatches<'static> {
//     let footnote = "Cargo commands (-x) are always executed before shell commands (-s). You can use the `-- command` style instead, note you'll need to use full commands, it won't prefix `cargo` for you.\n\nBy default, the workspace directories of your project and all local dependencies are watched, except for the target/ and .git/ folders. Your .ignore and .gitignore files are used to filter paths.".to_owned();

//     let mut app = App::new();

//     // Allow invocation of cargo-watch with both `cargo-watch watch ARGS`
//     // (as invoked by cargo) and `cargo-watch ARGS`.
//     let mut args: Vec<String> = env::args().collect();
//     args.insert(1, "watch".into());

//     let matches = match app.get_matches_from_safe_borrow(args) {
//         Ok(matches) => matches,
//         Err(err) => match err.kind {
//             ErrorKind::HelpDisplayed => {
//                 println!("{}", err);
//                 process::exit(0);
//             }

//             ErrorKind::VersionDisplayed => {
//                 println!();
//                 process::exit(0);
//             }

//             _ => app.get_matches(),
//         },
//     };

//     matches.subcommand.unwrap().matches
// }
