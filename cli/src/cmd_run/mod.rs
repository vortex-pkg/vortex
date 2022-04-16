#[path = "../run_script.rs"]
mod run_script;

use clap::ArgMatches;

use crate::cmd_run::run_script::run_script;

pub fn run(arg_matches: &ArgMatches) {
    run_script(arg_matches.value_of("script"));
}