use clap::ArgMatches;
use inquire::{error::InquireResult, Confirm, Text};
use node_semver::Version;
use owo_colors::colored::*;
use serde::Serialize;
use std::{borrow::Borrow, fs::File, io::prelude::*};
use validate_package_name::validate;

const TEST_COMMAND: &str = "echo \"Error: no test specified.\" && exit 1";
const START_COMMAND: &str = "node .";

/// Displays an error and exits
pub fn display_error(err: &str) {
    eprintln!("{} {}", "error:".red().bold(), err);
    std::process::exit(exitcode::SOFTWARE);
}

#[derive(Serialize)]
struct Scripts<'a> {
    start: &'a str,
    test: &'a str,
}

#[derive(Serialize)]
struct PackageJson<'a> {
    name: &'a str,
    version: &'a str,
    description: &'a str,
    author: &'a str,
    main: &'a str,
    r#type: &'a str,
    scripts: Scripts<'a>,
    license: &'a str,
}

impl PackageJson<'_> {
    fn as_string(&self) -> String {
        let str = serde_json::to_string_pretty(self);
        str.unwrap()
    }
}

fn write_defaults(dir_name: &str, file: &mut File) {
    let json = PackageJson {
        name: dir_name,
        version: "1.0.0",
        description: "",
        author: "",
        main: "index.js",
        r#type: "commonjs",
        scripts: Scripts {
            start: START_COMMAND,
            test: TEST_COMMAND,
        },
        license: "MIT",
    };
    file.write_all(json.as_string().as_bytes()).unwrap();
    std::process::exit(exitcode::OK)
}

pub fn init(matches: &ArgMatches) {
    let dir = std::env::current_dir().unwrap();
    let dir_name_cow = dir.file_name().unwrap().to_string_lossy();
    let dir_name: &str = dir_name_cow.borrow();
    let mut file = File::create("package.json").unwrap();

    match matches.is_present("yes") {
        true => write_defaults(dir_name, &mut file),
        false => {}
    }

    let name = Text::new("Package name:")
        .with_default(dir_name.borrow())
        .with_validator(&|input| match validate(input) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        })
        .prompt();

    let version = Text::new("Package version:")
        .with_default("1.0.0")
        .with_validator(&|input| match input.parse::<Version>() {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        })
        .prompt();

    let description = Text::new("Package description:").prompt();

    let author = Text::new("Package author(s):").prompt();

    let main = Text::new("Path to entry file:")
        .with_default("index.js")
        .prompt();

    let use_es_modules = Confirm::new("Use ES modules?")
        .with_default(false)
        .with_help_message("ES Module syntax: import { x } from 'foo'")
        .prompt();

    let test = Text::new("Test command (that you can run with `vortex test`)")
        .with_default(TEST_COMMAND)
        .prompt();

    let license = Text::new("License:")
        .with_default("MIT")
        .with_validator(&|input| match is_valid_license(input) {
            Ok(_) => Ok(()),
            _ => Err("License must be a valid SPDX license expression".to_string()),
        })
        .prompt();

    let json = PackageJson {
        name: name.handle_input_error(),
        version: version.handle_input_error(),
        description: description.handle_input_error(),
        author: author.handle_input_error(),
        license: license.handle_input_error(),

        main: main.handle_input_error(),
        r#type: if *use_es_modules.handle_input_error() {
            "module"
        } else {
            "commonjs"
        },

        scripts: Scripts {
            start: START_COMMAND,
            test: test.handle_input_error(),
        },
    };
    let raw = serde_json::to_string_pretty(&json).unwrap();
    println!("{raw}");
    match Confirm::new("Are you sure that you want to create package.json with these values?")
        .with_default(true)
        .prompt()
    {
        Ok(true) => {
            let mut file = File::create("package.json").unwrap();
            file.write_all(raw.as_bytes()).unwrap();
        }
        Ok(false) => {
            eprintln!("{}", "Exiting.".bold().red());
            std::process::exit(1);
        }
        Err(error) => {
            display_error(error.to_string().as_str());
        }
    };
}

fn is_valid_license(license: &str) -> Result<(), ()> {
    match spdx::Expression::parse(license) {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}

trait InquireResultTrait<T> {
    fn handle_input_error(&self) -> &T;
}
impl<T> InquireResultTrait<T> for InquireResult<T> {
    fn handle_input_error(&self) -> &T {
        match self {
            Ok(s) => s,
            Err(error) => {
                display_error(error.to_string().as_str());
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn valid_mit() {
        assert_eq!(is_valid_license("MIT"), Ok(()));
    }

    #[test]
    fn valid_isc() {
        assert_eq!(is_valid_license("ISC"), Ok(()));
    }

    #[test]
    fn invalid_your_mum() {
        assert_eq!(is_valid_license("your mum"), Err(()));
    }
}
