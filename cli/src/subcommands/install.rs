use clap::ArgMatches;
use install_npm::walk_dependencies;
use node_semver::Range;
use owo_colors::OwoColorize;

pub async fn install(_matches: &ArgMatches) {
    println!("📦 Installing dependencies...");
    let mut dependencies = walk_dependencies(
        &String::from("express"),
        Range::parse("4.18.1").unwrap(),
        "https://registry.npmjs.org",
    )
    .await
    .unwrap();
    dependencies.dedup();

    for dependency in dependencies.iter() {
        if dependency.dependencies.is_some() {
            for (key, value) in dependency.dependencies.as_ref().unwrap() {
                println!("{} {} {}", "+".green(), key, value);
            }
        }
    }
}
