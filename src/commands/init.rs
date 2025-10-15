use std::fs::File;
use std::io::Write;
use std::path::Path;
use super::Command;

pub struct InitCommand;

impl InitCommand {
    pub(crate) fn new() -> Self {
        InitCommand
    }

    fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
        let mut config_file = File::create("todohunt.toml")?;

        let config_content = r#"[general]
ignore = ["vendor", "node_modules", ".idea", ".vscode"]
ignore_file_types = [".env"]
work_dir = ["src"] # ./

[provider]
name = "github"
token = "${GITHUB_TOKEN}"
replace_todo = false
add_issue_number = true
add_issue_link = true

[export]
format = "markdown" # json
output = "todohunt.md"
open_links_in = "vscode" # idea
"#;

        config_file.write_all(config_content.as_bytes())?;
        Ok(())
    }
}

impl Command for InitCommand {
    fn name(&self) -> &'static str {
        "init"
    }

    fn handle(&self) -> i32 {
        println!("Initializing...");

        let config_file = "todohunt.toml";

        if Path::new(config_file).exists() {
            println!("Config file already exists: {}", config_file);
            return 0;
        }

        match InitCommand::create_default_config() {
            Ok(_) => println!("🎉 Created config file: {}", config_file),
            Err(err) => {
                eprintln!("❌ Failed to create config: {}", err);
                return 1;
            }
        }

        0
    }
}
