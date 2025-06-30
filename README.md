# Smart Tasker

[![Crates.io](https://img.shields.io/crates/v/smart-tasker)](https://crates.io/crates/smart-tasker)

Smart Tasker is a fast, lightweight command-line task tracker written in Rust. It lets you add, list, complete, and delete tasks directly from the terminal. All tasks are saved locally in a JSON file (`data/tasks.json`) — no database or external setup required.

---

## Install via Cargo

```bash
cargo install smart_tasker
```

Once installed, use it globally:

```bash
smart_tasker add "Write documentation"
smart_tasker list
smart_tasker complete 2
smart_tasker delete 1
```

---

## Features

- Add new tasks from the command line
- List all tasks with their completion status
- Mark tasks as completed
- Delete tasks by ID
- Simple local JSON storage

---

## Requirements

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

---

## Command Reference

| Command                       | Description                     |
|------------------------------|---------------------------------|
| `add "task desc"`            | Add a new task                  |
| `list`                       | Show all tasks                  |
| `complete <id>`              | Mark the task with given ID complete |
| `delete <id>`                | Delete the task with given ID   |

> Task IDs are shown in the list output. They start from 1.

---

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

---

## Links

- [Crates.io Package](https://crates.io/crates/smart-tasker)
- [GitHub Releases](https://github.com/webrowse/smart-cli-tasker/releases)

---

## License

This project is open source and freely usable under the MIT license.  
See `LICENSE` for details.

---

## Author

Created by Adarsh as a hands-on project while learning Rust.  
Feel free to fork, improve, or use it as a learning reference.
