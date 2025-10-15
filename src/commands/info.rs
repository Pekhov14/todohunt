use super::Command;

pub struct InfoCommand;

impl InfoCommand {
    pub(crate) fn new() -> Self {
        InfoCommand
    }

    fn display_help() -> () {
        println!("\n🦀 todohunt — hunting for TODOs\n");
        println!("\n version 0.1.0\n");
        println!("\nUsage: todohunt [COMMAND] [OPTIONS]\n");
        println!("Commands:");
        println!("  scan    Scan for TODOs in the project");
        println!("  sync    Sync issues with git provider");
        println!("  init    Create default config file");
        println!("  info    Information about commands and project");
        println!("  list    Show this help message\n");
        println!("Examples:");
        println!("  todohunt scan    # Scan project for TODOs");
        println!("  todohunt init    # Create config file");
        println!("  todohunt list    # Show in console all TODOs in project");
        println!("  todohunt list -e # Create file with all TODOs in project");
    }
}

impl Command for InfoCommand {
    fn name(&self) -> &'static str {
        "info"
    }

    fn handle(&self) -> i32 {
        Self::display_help();
        0
    }
}