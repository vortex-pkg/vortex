use std::process::{Command, ExitStatus};

pub fn run(script: &str, shell: &str) -> ExitStatus {
    let status = if cfg!(target_os = "windows") {
        Command::new(shell)
            .arg("/c")
            .arg(script)
            .status()
            .expect("failed to execute process")
    } else {
        Command::new(shell)
            .arg("-c")
            .arg(script)
            .status()
            .expect("failed to execute process")
    };
    status
}
