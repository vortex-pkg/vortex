use clap::ArgMatches;

#[path = "../run_script.rs"]
mod run_script;

pub fn test(matches: &ArgMatches) {
    run_script::run_script(
        "test",
        run_script::get_scripts(),
        matches.value_of("shell").unwrap(),
    );
}
