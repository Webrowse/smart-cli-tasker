mod commands;
mod models;
mod storage;

use commands::{add, complete, delete, list};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Smart Tasker")]
#[command(about = "CLI to manage tasks")]
#[command(version)]
#[command(author = "Adarsh")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { desc: String },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { desc }) => add(desc),
        Some(Commands::List) => list(),
        Some(Commands::Complete { id }) => complete(id),
        Some(Commands::Delete { id }) => delete(id),
        None => {
            eprintln!("No command given. Use `--help` to see help");
        }
    }
}
