use super::Command;

pub struct SyncCommand;

impl SyncCommand {
    pub fn new() -> Self {
        SyncCommand
    }
}

impl Command for SyncCommand {
    fn name(&self) -> &'static str {
        "sync"
    }

    fn handle(&self) -> i32 {
        println!("🔄 Syncing TODOs with remote provider...");
        0
    }
}
