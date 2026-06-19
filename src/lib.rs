use std::{
    fmt,
    fs,
};

use serde::{
    Deserialize,
    Serialize
};

pub enum Command {
    Add,
    List,
    Done,
    Undone,
    Remove,
}

pub struct Config {
    pub todo: String,
    pub command: Command,
    pub argument: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Use: todo <command> [\"argument\"]");
        }

        let todo = args[1].to_lowercase();
        let command = match args[2].to_lowercase().as_str() {
            "add" => Command::Add,
            "list" => Command::List,
            "done" => Command::Done,
            "undone" => Command::Undone,
            "remove" => Command::Remove,
            _ => return Err("Unknown Command")
        };

        // get(3) returns `Some(&String)` if index 3 exists, otherwise `None`.
        // cloned() converts `Option<&String>` into `Option<String>`.
        let argument = args.get(3).cloned();

        if todo != "todo" {
            return Err("Invalid command. Use: todo <command> [\"argument\"]");
        }

        Ok(Config {
            todo,
            command,
            argument,
        })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let mut todo_list = TodoList::load();

    match config.command {
        Command::Add => {
            let description = config
                .argument
                .ok_or("Provide a description of the task. Use: todo add <\"description\">")?;
            
            todo_list.create_task(description);
        }
        
        Command::List => {
            todo_list.list_all_tasks()
        }
        
        Command::Done => {
            let task_id = todo_list.get_task_id(config)?;

            todo_list.done_task(task_id)?;
        },

        Command::Undone => {
            let task_id = todo_list.get_task_id(config)?;

            todo_list.undone_task(task_id)?;
        }
        
        Command::Remove => {
            let task_id = todo_list.get_task_id(config)?;

            todo_list.remove_task(task_id)?;
        }
    }

    todo_list.save()
        .map_err(|_|"Failed to save task")?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    Completed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    status: Status,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {} - {}", self.id, self.status, self.description)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    tasks: Vec<Task>,
}

const TASKS_FILE_PATH: &str = "tasks.json";

impl TodoList {

    pub fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }

    pub fn load() -> Self {
        match fs::read_to_string(self::TASKS_FILE_PATH) {
            Ok(content) => {
                serde_json::from_str(&content)
                    .unwrap_or_else(|_| Self::new())
            }

            Err(_) => Self::new(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;

        fs::write(TASKS_FILE_PATH, json)?;

        Ok(())
    }

    fn generate_id(&self) -> u32 {
        self.tasks
            .iter()
            .map(|task| task.id)
            .max()
            .unwrap_or(0)
            + 1
    }

    pub fn get_task_id(&self, config: Config) -> Result<u32, &'static str> {
        let argument = config.argument.ok_or("Provide task Id")?;
        
        let task_id = argument
                .parse::<u32>()
                .map_err(|_| "Invalid task id")?;

        Ok(task_id)
    }

    pub fn create_task(&mut self, description: String) {
        let task = Task {
            id: self.generate_id(),
            description,
            status: Status::Pending,
        };

        self.tasks.push(task)
    }

    pub fn list_all_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No task found.");
            return;
        }

        for task in &self.tasks {
            println!("{task}");
        }
    }

    pub fn done_task(&mut self, task_id: u32) -> Result<(), &'static str> {
        match self.find_task_mut(task_id) {
            Some(task) => {
                task.status = Status::Completed;
                Ok(())
            }
            None => return Err("Task not found"),
        }
    }

    pub fn undone_task(&mut self, task_id: u32) -> Result<(), &'static str> {
        match self.find_task_mut(task_id) {
            Some(task) => {
                task.status = Status::Pending;
                Ok(())
            }
            None => return Err("Task not found"),
        }
    }

    pub fn remove_task(&mut self, task_id: u32) -> Result<(), &'static str>{
        match self.find_task_index(task_id) {
            Some(index) => {
                self.tasks.remove(index);
                Ok(())
            }
            None => return Err("Task not found"),
        }
    }

    pub fn find_task(&self, task_id: u32) -> Option<&Task> {
        self.tasks
            .iter()
            .find(|task| task.id == task_id)
    }

    pub fn find_task_mut(&mut self, task_id: u32) -> Option<&mut Task> {
        self.tasks
            .iter_mut()
            .find(|task| task.id == task_id)
    }

    pub fn find_task_index(&self, task_id: u32) -> Option<usize> {
        self.tasks
            .iter()
            .position(|task| task.id == task_id)
    }
}
