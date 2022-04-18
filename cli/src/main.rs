use clap::{command, Arg, Command};
use panic_hook::hook as panic_hook;

mod init;
mod run;
mod test;

const SHELL: &str = if cfg!(target_os = "windows") {
    "cmd"
} else {
    "sh"
};

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
        Some(("init", matches)) => init::init(matches),
        Some(("run-script", matches)) => run::run(matches),
        Some(("test", matches)) => test::test(matches),
        _ => unreachable!(
            "Command is not defined in the command list, but subcommand_required is enabled"
        ),
    }

    Ok(())
}
