use colored::Colorize;

/// Displays an error and exits
pub fn display_error(err: &str) {
    eprintln!("{} {}", "error:".red().bold(), err);
    std::process::exit(exitcode::SOFTWARE);
}