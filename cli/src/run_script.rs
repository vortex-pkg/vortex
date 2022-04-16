use colored::*;
use json::JsonValue;
use std::fs;
use std::path::Path;

fn invalid_data(err: &str) {
    println!("{} {}", "error:".red().bold(), err);
    std::process::exit(exitcode::DATAERR);
}

/// Runs a package.json script
pub fn run_script(name: Option<&str>) {
    let scripts = get_scripts();
    if scripts.is_null() {
        invalid_data("scripts object does not exist in package.json");
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

    let script = &scripts[name];
    if script.is_null() {
        invalid_data(format!("script `{}` does not exist", name).as_str());
    }

    if !script.is_string() {
        invalid_data(format!("script `{}` exists but it is not a string", name).as_str())
    }

    let status = run_in_shell::run(script.to_string().as_str());
    if !status.success() {
        match status.code() {
            Some(code) => {
                eprintln!("{} process didn't exit successfully: (exit code: {})", "error:".red().bold(), code);
                std::process::exit(code);
            },
            None => eprintln!("{} Process terminated by signal", "error:".red().bold())
        }
    }
}

pub fn get_scripts() -> JsonValue {
    let path = Path::new("package.json");
    if !path.exists() {
        eprintln!(
            "{} package.json does not exist. Run 'vortex init' to create one.",
            "error:".red().bold()
        );
        std::process::exit(exitcode::NOINPUT);
    }

    let parsed = match json::parse(fs::read_to_string("package.json").unwrap().as_str()) {
        Ok(json) => json,
        _ => {
            invalid_data("package.json is invalid");
            unreachable!();
        }
    };

    parsed["scripts"].clone()
}
