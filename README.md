# Smart Tasker

Smart Tasker is a command-line task tracker built with Rust. It lets you add, list, complete, and delete tasks directly from the terminal. All tasks are stored in a local JSON file (`data/tasks.json`), so there is no need for a database or external setup.

## Features

- Add new tasks from the command line
- List all tasks with their completion status
- Mark tasks as completed
- Delete tasks by ID
- Simple and fast local storage using JSON

## Requirements

- Rust and Cargo installed  
  (Install from https://www.rust-lang.org/tools/install)

## How to Use

```bash
# Clone the repo
git clone https://github.com/yourusername/smart-tasker.git
cd smart-tasker

# Run any command with cargo
cargo run -- add "Write documentation"
cargo run -- list
cargo run -- complete 2
cargo run -- delete 1
```

## Command Reference

| Command             | Description                          |
|---------------------|--------------------------------------|
| `add "task desc"`   | Add a new task                       |
| `list`              | Show all tasks                       |
| `complete <id>`     | Mark the task with given ID complete |
| `delete <id>`       | Delete the task with given ID        |

Task IDs are shown in the `list` output. Start from 1.

## File Structure

```
src/
├── main.rs            # CLI entry point and Clap logic
├── commands/          # add.rs, list.rs, complete.rs, delete.rs
├── models/            # task.rs (Task struct, derives Serde)
├── storage/           # file.rs (load, save, show)
data/
└── tasks.json         # Local JSON file storing tasks
```

## License

This project is open source and freely usable under the MIT license.  
See `LICENSE` for details.

## Author

Created by Adarsh as a hands-on project while learning Rust.  
Feel free to fork, improve, or use it as a learning reference.
