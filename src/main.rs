use std::fs::File;
use std::io::{Write};
use std::path::Path;


fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_file = File::create("todohunt.toml")?;

    let config_content = r#"[general]
ignore = ["vendor", "node_modules", ".idea", ".vscode"]
ignore_file_types = [".env"]

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = "todohunt.toml";

    if Path::new(config_file).exists() {
        // read config
        println!("Config file already exists: {}", config_file);
    } else {
        // Init config file
        create_default_config()?;
        println!("Creating config file: {}", config_file);
    }

    Ok(())
}
