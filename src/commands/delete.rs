use crate::storage::{load_tasks, save_tasks, show_tasks};

pub fn delete(id: i32) {
    let mut tasks = load_tasks();
    if id == 0 || id > tasks.len() as i32 {
        println!("Invalid input, nothing was deleted, \nTry Again");
        show_tasks();
        return;
    }
    let index = (id - 1) as usize;
    let deleted = tasks[index as usize].desc.clone();
    tasks = tasks
        .into_iter()
        .enumerate()
        .filter_map(|(i, task)| if i != index {Some(task)} else{None})
        .collect();
    save_tasks(&tasks);
    println!(r#"Deleted Task: "{}" successfully"#, deleted);
    show_tasks();
}
