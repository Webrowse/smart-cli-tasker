use crate::models::Task;
use crate::storage::{load_tasks, save_tasks, show_tasks};

pub fn add(desc: String) {
    let mut tasks = load_tasks();
    tasks.push(Task {
        desc,
        completed: false,
    });
    save_tasks(&tasks);
    println!("Task added");
    show_tasks();
}
