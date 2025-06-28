
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

const DATA_FILE: &str = "data/tasks.json";

#[derive(Parser)]
#[command(name = "Smart Tracker")]
#[command(about = "CLI to manage tasks")]
struct Cli{
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands{
    Add { desc: String },
    List
}

#[derive(Serialize, Deserialize, Debug)]
struct Task{
    desc: String,
    completed: bool
}

fn main(){
    let cli = Cli::parse();

    match cli.command{
        Commands::Add { desc } => {
            let mut tasks = load_tasks();
            tasks.push(Task {
                desc,
                completed: false
            });
            save_tasks(&tasks);
            println!("Task added");
        },
        Commands::List => {
            let tasks = load_tasks();
            for (i, task) in tasks.iter().enumerate(){
                let status = if task.completed { "[x]" } else { "[ ]"};
                println!("{} {} {} ", i+1, status, task.desc);
            }
        }
    }
}

fn load_tasks() -> Vec<Task>{
    if !Path::new(DATA_FILE).exists(){
        return Vec::new()
    }

    let file = File::open(DATA_FILE).expect("failed to open DATA_FILE");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &Vec<Task>) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(DATA_FILE)
        .expect("failed to write file");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).expect("Failed to serialise task");
}