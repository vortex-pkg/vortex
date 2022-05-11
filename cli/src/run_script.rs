use clap::ArgMatches;
use owo_colors::colored::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{collections::HashMap, fs};

pub fn invalid_data(err: &str) {
    eprintln!("{} {}", "error:".red().bold(), err);
    std::process::exit(exitcode::DATAERR);
}

#[derive(Serialize, Deserialize)]
pub struct Contents {
    scripts: Option<HashMap<String, String>>,
}

/// Runs a package.json script
pub fn run_script(name: &str, scripts: HashMap<String, String>, shell: &str) {
    let script_option = &scripts.get(name);
    let script = match script_option {
        Some(s) => s,
        None => {
            invalid_data(format!("script `{}` does not exist", name).as_str());
            unreachable!();
        }
    };

    let status = run_in_shell::run(&script, shell);
    if !status.success() {
        match status.code() {
            Some(code) => {
                eprintln!(
                    "{} process didn't exit successfully: (exit code: {})",
                    "error:".red().bold(),
                    code
                );
                if !Path::new("node_modules").is_dir() {
                    eprintln!(
                        "{} you might need to run `vortex install` and try again",
                        "help:".cyan().bold()
                    )
                }
                std::process::exit(code);
            }
            None => eprintln!("{} Process terminated by signal", "error:".red().bold()),
        }
    }
}

pub fn get_scripts() -> Option<HashMap<String, String>> {
    let path = Path::new("package.json");
    if !path.exists() {
        eprintln!(
            "{} package.json does not exist. Run 'vortex init' to create one.",
            "error:".red().bold()
        );
        std::process::exit(exitcode::NOINPUT);
    }

    let contents = match fs::read_to_string("package.json") {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "{} failed to read package.json: {}",
                "error:".red().bold(),
                e
            );
            std::process::exit(exitcode::IOERR)
        }
    };

    let parsed: Contents = match serde_json::from_str(contents.as_str()) {
        Ok(json) => json,
        _ => {
            invalid_data("package.json is invalid");
            unreachable!();
        }
    };

    parsed.scripts
}

#[allow(dead_code)]
pub fn alias(name: &str, matches: &ArgMatches) {
    let scripts = match get_scripts() {
        Some(scripts) => scripts,
        None => {
            invalid_data("scripts object does not exist in package.json");
            unreachable!()
        }
    };

    run_script(name, scripts, matches.value_of("shell").unwrap());
}
