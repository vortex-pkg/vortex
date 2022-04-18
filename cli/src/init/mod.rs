use clap::ArgMatches;
use colored::Colorize;
use display_error::display_error;
use inquire::{error::InquireResult, Confirm, Text};
use json::object;
use std::{borrow::Borrow, fs::File, io::prelude::*};
use validate_package_name::validate;

pub fn init(matches: &ArgMatches) {
    let dir = std::env::current_dir().unwrap();
    let dir_name_cow = dir.file_name().unwrap().to_string_lossy();
    let dir_name: &str = dir_name_cow.borrow();

    match matches.is_present("yes") {
        true => {
            let json = object!{
                name: dir_name,
                version: "1.0.0",
                description: "",
                author: "",
                main: "index.js",
                scripts: {
                    test: "echo \"Error: no test specified\" && exit 1"
                },
                license: "MIT"
            };
            let mut file = File::create("package.json").unwrap();
            file.write_all(json.pretty(4).as_bytes()).unwrap();
            std::process::exit(exitcode::OK);
        },
        false => {}
    }

    let mut package_json = object! {};
    package_json["name"] = handle_input_error(
        Text::new("Package name:")
            .with_default(dir_name.borrow())
            .with_validator(&|input| match validate(input) {
                Ok(_) => Ok(()),
                Err(error) => Err(error.to_string()),
            })
            .prompt(),
    )
    .into();

    package_json["version"] = handle_input_error(
        Text::new("Package version:")
            .with_default("1.0.0")
            .with_validator(&|input| match semver::Version::parse(input) {
                Ok(_) => Ok(()),
                Err(error) => Err(error.to_string()),
            })
            .prompt(),
    )
    .into();

    package_json["description"] =
        handle_input_error(Text::new("Package description:").prompt()).into();

    package_json["author"] = handle_input_error(Text::new("Package author(s):").prompt()).into();

    package_json["main"] = handle_input_error(
        Text::new("Path to entry file:")
            .with_default("index.js")
            .prompt(),
    )
    .into();

    match Confirm::new("Use ES modules?")
        .with_default(false)
        .with_help_message("ES Module syntax: import { x } from 'foo'")
        .prompt()
    {
        Ok(true) => package_json["type"] = "module".into(),
        Ok(false) => {}
        Err(error) => display_error(error.to_string().as_str()),
    };

    let test = handle_input_error(
        Text::new("Test command (that you can run with `vortex test`)").prompt(),
    );
    package_json["scripts"]["test"] = if !test.trim().is_empty() {
        test.into()
    } else {
        "echo \"Error: no test specified\" && exit 1".into()
    };

    package_json["license"] = handle_input_error(
        Text::new("License:")
            .with_default("MIT")
            .with_validator(&|input| match is_valid_license(input) {
                Ok(_) => Ok(()),
                _ => Err("License must be a valid SPDX license expression".to_string()),
            })
            .prompt(),
    )
    .into();

    let raw = package_json.pretty(4);
    match Confirm::new("Are you sure that you want to create package.json with these values?")
        .with_default(true)
        .with_help_message(&raw)
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

fn handle_input_error(result: InquireResult<String>) -> String {
    match result {
        Ok(s) => s,
        Err(error) => {
            display_error(error.to_string().as_str());
            unreachable!()
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
