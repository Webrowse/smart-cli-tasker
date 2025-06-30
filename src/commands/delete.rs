use crate::storage::{load_tasks, save_tasks, show_tasks};

pub fn delete(id: usize) {
    let mut tasks = load_tasks();
    if id == 0 || id > tasks.len() {
        println!("Invalid input, nothing was deleted, \nTry Again");
        show_tasks();
        return;
    }
    let index = id - 1;
    tasks = tasks
        .into_iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, task)| task)
        .collect();
    save_tasks(&tasks);
    let deleted = &tasks[index].desc;
    println!("Deleted {:?} successfully", deleted);
    show_tasks();
}
