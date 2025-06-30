use crate::storage::{load_tasks, save_tasks, show_tasks};

pub fn complete(id: i32) {
    let mut tasks = load_tasks();
    if id == 0 || id > tasks.len() as i32 {
        println!("Invalid task ID");
        show_tasks();
        return;
    }
    let index = (id - 1) as usize;
    tasks[index].completed = true;
    save_tasks(&tasks);
    show_tasks();
}
