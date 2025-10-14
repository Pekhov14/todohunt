use std::{fs, process};
use std::fs::File;
use std::io::{BufRead, Read, Write};
use std::path::Path;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = "todohunt.toml";

    if Path::new(config_file).exists() {
        // read config
        println!("Config file already exists: {}", config_file);
    } else {
        create_default_config()?;
        println!("Creating config file: {}", config_file);
    }

    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        display_help();
        process::exit(0);
    }

    let command = &args[1];

    dbg!(command);

    match command.as_str() {
        "scan" => { println!("Scanning project"); },
        "init" => { println!("Initializing..."); },
        "info" => display_help(),
        _ => { eprint!("{} is not a valid provided command", command); process::exit(1); }
    }


    Ok(())
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

fn display_help() -> () {
    println!("\n🦀 todohunt — hunting for TODOs\n");
    println!("\nUsage: todohunt [COMMAND] [OPTIONS]\n");
    println!("Commands:");
    println!("  scan    Scan for TODOs in the project");
    println!("  init    Create default config file");
    println!("  help    Show this help message\n");
    println!("Examples:");
    println!("  todohunt scan    # Scan project for TODOs");
    println!("  todohunt init    # Create config file");
}
