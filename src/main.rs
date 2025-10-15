mod commands;
use commands::*;

use std::{env, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("There is no such command. To see all commands, enter:\
         todohunt help ");
        process::exit(0);
    }

    let command = &args[1];

    let exit_code = match command.as_str() {
        "scan" => ScanCommand::new().handle(),
        "sync" => SyncCommand::new().handle(),
        "init" => InitCommand::new().handle(),
        "info" => InfoCommand::new().handle(),
        "list" => ListCommand::new().handle(),
        _ => {
            eprint!("\x1b[31m {} \x1b[0m is not a valid provided command", command);
            1
        }
    };

    process::exit(exit_code);
}
