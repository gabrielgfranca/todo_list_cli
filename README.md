# Todo List CLI

A simple command-line Todo List application written in Rust for learning and practicing core Rust concepts after completing Chapter 12 of *The Rust Programming Language*.

This project focuses on applying Rust fundamentals in a real application, including:

* Structs and Enums
* Ownership and Borrowing
* Error Handling with `Result` and `Option`
* Collections (`Vec`)
* Modules
* File I/O
* Serialization and Deserialization with Serde
* Command-Line Argument Parsing
* Project Organization and Refactoring

## Features

* Create tasks
* List all tasks
* Mark tasks as completed
* Mark tasks as pending again
* Remove tasks
* Persist tasks between executions using a JSON file
* Automatically generate unique task IDs

## Project Structure

```text
src/
├── main.rs
├── lib.rs
├── app.rs
├── cli.rs
├── task.rs
└── todo_list.rs
```

### Modules

| Module         | Responsibility                                        |
| -------------- | ----------------------------------------------------- |
| `cli.rs`       | Command-line argument parsing and command definitions |
| `task.rs`      | Task and status models                                |
| `todo_list.rs` | Todo list business logic and persistence              |
| `app.rs`       | Application workflow and command execution            |
| `main.rs`      | Application entry point                               |
| `lib.rs`       | Module exports                                        |

## Installation

### Clone the repository

```bash
git clone <repository-url>
cd todo_list_cli
```

### Build the project

```bash
cargo build
```

### Run the application

```bash
cargo run -- todo <command> [argument]
```

## Commands

### Add a task

```bash
cargo run -- todo add "Study Rust"
```

Example output:

```text
Task created successfully.
```

### List tasks

```bash
cargo run -- todo list
```

Example output:

```text
[1] Pending - Study Rust
[2] Completed - Finish assignment
```

### Mark task as completed

```bash
cargo run -- todo done 1
```

### Mark task as pending

```bash
cargo run -- todo undone 1
```

### Remove a task

```bash
cargo run -- todo remove 1
```

## Data Persistence

Tasks are stored locally in a JSON file:

```text
tasks.json
```

Example:

```json
{
  "tasks": [
    {
      "id": 1,
      "description": "Study Rust",
      "status": "Pending"
    },
    {
      "id": 2,
      "description": "Build Todo CLI",
      "status": "Completed"
    }
  ]
}
```

The application automatically:

1. Loads tasks from `tasks.json` when it starts.
2. Applies the requested command.
3. Saves the updated data back to `tasks.json`.

## Dependencies

### Serde

Used for serialization and deserialization.

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Learning Goals

This project was created to reinforce practical Rust skills and prepare for more advanced projects.

Concepts practiced include:

* Struct and Enum design
* Module organization
* CLI application development
* Error propagation with `?`
* File management
* Data persistence
* JSON serialization
* Mutable and immutable references
* Pattern matching with `match`
* Iterator methods (`find`, `position`, `map`, `max`)

## License

This project is intended for educational purposes and personal learning.
