# Smart Tasker

Smart Tasker is a command-line task tracker built with Rust. It lets you add, list, complete, and delete tasks directly from the terminal. All tasks are stored in a local JSON file (`data/tasks.json`), so there is no need for a database or external setup.

---

## Features

- Add new tasks from the command line
- List all tasks with their completion status
- Mark tasks as completed
- Delete tasks by ID
- Simple and fast local storage using JSON

---

## Requirements

- Rust and Cargo installed  
  (Install from https://www.rust-lang.org/tools/install)

---

## How to Use

```bash
# Clone the repo
git clone https://github.com/Webrowse/smart-cli-tasker.git
cd smart-cli-tasker
```
# Run any command with cargo
- cargo run -- add "Write documentation"
- cargo run -- list
- cargo run -- complete 2
- cargo run -- delete 1
