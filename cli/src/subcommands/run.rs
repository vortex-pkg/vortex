use clap::ArgMatches;
use owo_colors::colored::*;
use crate::run_script::*;

pub fn run(arg_matches: &ArgMatches) {
    let name = arg_matches.value_of("script");
    let scripts = match get_scripts() {
        Some(scripts) => scripts,
        None => {
            invalid_data("scripts object does not exist in package.json");
            unreachable!()
        }
    };

    let name = match name {
        Some(name) => name,
        None => {
            eprintln!("All scripts:");
            for (key, value) in scripts {
                eprintln!("{}\n    {}", key.cyan().bold(), value);
            }
            std::process::exit(exitcode::USAGE);
        }
    };

    run_script(name, scripts, arg_matches.value_of("shell").unwrap());
}
