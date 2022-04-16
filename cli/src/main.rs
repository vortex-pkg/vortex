use clap::{command, Arg, Command};
use panic_hook::hook as panic_hook;

mod cmd_init;
mod cmd_run;

fn main() -> Result<(), ()> {
    panic_hook();
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Creates a new package.json file.")
                .arg(Arg::new("yes").short('y').long("yes"))
                .aliases(&["create", "innit"]),
        )
        .subcommand(
            Command::new("run-script")
                .about("Run a script defined in the package.json file.")
                .after_help("test")
                .arg(Arg::new("script"))
                .aliases(&["run", "rum", "urn"]),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", matches)) => cmd_init::init(matches),
        Some(("run-script", matches)) => cmd_run::run(matches),
        _ => unreachable!(
            "Command is not defined in the command list, but subcommand_required is enabled"
        ),
    }

    Ok(())
}
