use colored::Colorize;
use std::{
    panic::{self, PanicInfo},
    path::PathBuf,
};
pub mod report;
use report::{Method, Report};

/// Setup a panic hook for Vortex. Based on `human-panic`
pub fn hook() {
    panic::set_hook(Box::new(|panic_info| {
        eprintln!(
"{}
Vortex had a problem and crashed. We'd appreciate you sending us a crash report to help us diagnose the problem!

We take privacy seriously, and do not perform any automated error collection. In order to improve Vortex, we rely on people to submit crash reports.

Please report this issue at {}, and attach the crash report.
It's located at {}.

{}",
                    "Well, this is awkward.".red().bold(),
                    format!("{}/issues", env!("CARGO_PKG_REPOSITORY")).cyan().underline().bold(),
                    match handle_dump(env!("CARGO_PKG_VERSION"), panic_info).as_ref() {
                        Some(path) => format!("{}", path.display().to_string().cyan().underline().bold()),
                        None => "<Failed to save crash report, please include this in your issue>".to_string()
                    },
                    "Thanks 👍".green().bold(),
                );
    }));
}

/// Utility function which will handle dumping information to disk
pub fn handle_dump(version: &str, panic_info: &PanicInfo) -> Option<PathBuf> {
    let mut expl = String::new();

    #[cfg(feature = "nightly")]
    let message = panic_info.message().map(|m| format!("{}", m));

    #[cfg(not(feature = "nightly"))]
    let message = match (
        panic_info.payload().downcast_ref::<&str>(),
        panic_info.payload().downcast_ref::<String>(),
    ) {
        (Some(s), _) => Some(s.to_string()),
        (_, Some(s)) => Some(s.to_string()),
        (None, None) => None,
    };

    let cause = match message {
        Some(m) => m,
        None => "Unknown".into(),
    };

    match panic_info.location() {
        Some(location) => expl.push_str(&format!(
            "Panic occurred in file '{}' at line {}\n",
            location.file(),
            location.line()
        )),
        None => expl.push_str("Panic location unknown.\n"),
    }

    let report = Report::new("vortex", version, Method::Panic, expl, cause);

    match report.persist() {
        Ok(f) => Some(f),
        Err(_) => {
            eprintln!("{}", report.serialize().unwrap());
            None
        }
    }
}
