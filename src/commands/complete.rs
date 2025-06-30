use crate::storage::{load_tasks, save_tasks, show_tasks};

pub fn complete(id: u32) {
    let mut tasks = load_tasks();
    if id == 0 || id > tasks.len() as u32 {
        println!("Invalid task ID");
        show_tasks();
        return;
    }
    let index = (id - 1) as usize;

    if tasks[index].completed {
        println!(r#"Task already completed: "{}""#, tasks[index].desc);
    }
    else{
        tasks[index].completed = true;
        save_tasks(&tasks);
        println!(r#"Marked as complete: "{}" successfully."#, tasks[index].desc);
        show_tasks();

    }
}
