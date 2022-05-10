#[path = "../run_script.rs"]
mod run_script;

use clap::ArgMatches;
use owo_colors::colored::*;

pub fn run(arg_matches: &ArgMatches) {
    let name = arg_matches.value_of("script");
    let scripts = match run_script::get_scripts() {
        Some(scripts) => scripts,
        None => {
            run_script::invalid_data("scripts object does not exist in package.json");
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

    run_script::run_script(name, scripts, arg_matches.value_of("shell").unwrap());
}
