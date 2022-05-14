use clap::{Command, Arg, crate_version, ArgMatches};

const SHELL: &str = if cfg!(target_os = "windows") {
    "cmd"
} else {
    "sh"
};

pub fn get_matches() -> ArgMatches {
    Command::new("vortex")
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
        .subcommand(
            Command::new("install")
                .about("Install a package.")
                .aliases(&["i", "add"]),
        )
        .get_matches()
}
