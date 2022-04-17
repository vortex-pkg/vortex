#[path = "../run_script.rs"]
mod run_script;

use clap::ArgMatches;

pub fn run(arg_matches: &ArgMatches) {
    run_script::run_script(arg_matches.value_of("script"));
}