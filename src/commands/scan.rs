use super::Command;

pub struct ScanCommand;

impl ScanCommand {
    pub fn new() -> Self {
        ScanCommand
    }
}

impl Command for ScanCommand {
    fn name(&self) -> &'static str {
        "scan"
    }

    fn handle(&self) -> i32 {
        println!("🔍 Scanning project for TODOs...");

        // todo: read souses folders like a /src or ./
        // go by files maybe recursive or with iterators
        // search in file 'todo', 'TODO' maybe with regex
        // save to cache path to file, line where searched todo, date time file changed

        0
    }
}
