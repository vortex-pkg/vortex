use clap::{crate_version, Arg, Command};
use owo_colors::colored::*;

mod panic_hook;
pub mod run_script;

mod subcommands;
use crate::subcommands::{init::init, run::run};

const SHELL: &str = if cfg!(target_os = "windows") {
    "cmd"
} else {
    "sh"
};

#[tokio::main]
async fn main() -> Result<(), ()> {
    if cfg!(debug_assertions) {
        if color_eyre::install().is_err() {
            eprintln!(
                "{} failed to install {} panic hook, using release {}",
                "warn:".yellow().bold(),
                "color-eyre".italic(),
                "panic_hook".italic()
            )
        }
    } else {
        panic_hook::hook();
    }

    let matches = Command::new("vortex")
        .version(crate_version!())
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Creates a new package.json file.")
                .arg(Arg::new("yes").short('y').long("yes"))
                .arg(Arg::new("shell").long("script-shell").default_value(SHELL))
                .aliases(&["create", "innit"]),
        )
        .subcommand(
            Command::new("run-script")
                .about("Run a script defined in the package.json file.")
                .arg(Arg::new("script"))
                .arg(Arg::new("shell").long("script-shell").default_value(SHELL))
                .aliases(&["run", "rum", "urn"]),
        )
        .subcommand(
            Command::new("test")
                .about("Test a package.")
                .arg(Arg::new("shell").long("script-shell").default_value(SHELL))
                .aliases(&["tst", "t"]),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", matches)) => init(matches),
        Some(("run-script", matches)) => run(matches),
        Some(("test", matches)) => run_script::alias("test", matches),
        _ => unreachable!(
            "Command is not defined in the command list, but subcommand_required is enabled"
        ),
    }

    Ok(())
}
