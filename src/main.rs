use std::fs;
use std::fs::File;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todohunt", version = "0.1.0")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Scan,
    Sync,
}

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Commands::Init => initialization(),
        Commands::Scan => todo!(),
        Commands::Sync => todo!(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn initialization() -> std::io::Result<()> {
    if fs::exists(".todohunt")? {
        println!("Already initialized");
        return Ok(());
    }

    fs::create_dir(".todohunt")?;

    File::create(".todohunt/cache.json")?;
    File::create(".todohunt/issues.json")?;
    File::create(".todohunt/todos.json")?;

    println!("Initialized .todohunt/");
    Ok(())
}
