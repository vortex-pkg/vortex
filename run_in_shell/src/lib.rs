use std::process::{Command, ExitStatus};

pub fn run(script: &str) -> ExitStatus {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg(script)
            .status()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(script)
            .status()
            .expect("failed to execute process")
    };
    status
}
