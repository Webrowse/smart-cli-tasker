use crate::models::Task;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

const DATA_FILE: &str = "data/tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if !Path::new(DATA_FILE).exists() {
        return Vec::new();
    }

    let file = File::open(DATA_FILE).expect("failed to open DATA_FILE");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(DATA_FILE)
        .expect("failed to write file");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).expect("Failed to serialise task");
}

pub fn show_tasks() {
    let tasks = load_tasks();
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.completed { "[x]" } else { "[ ]" };
        println!("{} {} {} ", i + 1, status, task.desc);
    }
}
