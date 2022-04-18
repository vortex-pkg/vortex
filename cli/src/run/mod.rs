#[path = "../run_script.rs"]
mod run_script;

use colored::*;
use clap::ArgMatches;

pub fn run(arg_matches: &ArgMatches) {
    let name = arg_matches.value_of("script");
    let scripts = run_script::get_scripts();
    if scripts.is_null() {
        run_script::invalid_data("scripts object does not exist in package.json");
    }

    let name = match name {
        Some(name) => name,
        None => {
            eprintln!("All scripts:");
            for (key, value) in scripts.entries() {
                if value.is_string() {
                    eprintln!("{}\n    {}", key.cyan().bold(), value.as_str().unwrap());
                } else {
                    eprintln!("{}\n    Invalid script (type != string)", key.red().bold());
                }
            }
            std::process::exit(exitcode::USAGE);
        }
    };

    run_script::run_script(
        name,
        scripts,
        arg_matches.value_of("shell").unwrap(),
    );
}
