use owo_colors::colored::*;

mod panic_hook;
pub mod run_script;

mod subcommands;
use crate::subcommands::{init::init, install::install, run::run};

mod get_matches;
use crate::get_matches::get_matches;

#[tokio::main]
async fn main() -> Result<(), ()> {
    panic_hook::install();

    let matches = get_matches();

    match matches.subcommand() {
        Some(("init", matches)) => init(matches),
        Some(("run-script", matches)) => run(matches),
        Some(("test", matches)) => run_script::alias("test", matches),
        Some(("install", matches)) => install(matches).await,
        _ => unreachable!(
            "Command is not defined in the command list, but subcommand_required is enabled"
        ),
    }

    Ok(())
}
