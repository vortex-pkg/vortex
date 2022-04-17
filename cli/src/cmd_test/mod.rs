#[path = "../run_script.rs"]
mod run_script;

pub fn test() {
    run_script::run_script(Some("test"));
}