
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

const DATA_FILE: &str = "data/tasks.json";

#[derive(Parser)]
#[command(name = "Smart Tasker")]
#[command(about = "CLI to manage tasks")]
#[command(version)]
#[command(author = "Adarsh")]
struct Cli{
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands{
    Add { desc: String },
    List,
    Completed { id: usize },
    Delete { id: usize }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            show_tasks();
        },
        Commands::List => {
            show_tasks();
        },
        Commands::Completed { id } => {
            let mut tasks = load_tasks();
            if id == 0 || id > tasks.len(){
                println!("Invalid task ID");
                return;
            }
            let index = id - 1;
            tasks[index].completed = true;
            save_tasks(&tasks);
            show_tasks();
        },
        Commands::Delete { id } => {
            let tasks = load_tasks();
            if id == 0 || id > tasks.len(){
                println!("Invalid input");
                return;
            }
            let index = id - 1;
            let mut after_delete: Vec<Task> = Vec::new();
            for (i, value) in tasks.iter().clone().enumerate(){
                if i!= index {
                    after_delete.push(value.clone());
                }
            }
            save_tasks(&after_delete);
            show_tasks();
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

fn show_tasks(){
    let tasks = load_tasks();
            for (i, task) in tasks.iter().enumerate(){
                let status = if task.completed { "[x]" } else { "[ ]"};
                println!("{} {} {} ", i+1, status, task.desc);
            }
}