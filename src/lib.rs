use std::fmt;

pub enum Command {
    Add,
    List,
    Done,
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
            return Err("Not enough arguments. Use: todo <command> [argument]");
        }

        let todo = args[1].to_lowercase();
        let command = match args[2].to_lowercase().as_str() {
            "add" => Command::Add,
            "list" => Command::List,
            "done" => Command::Done,
            "remove" => Command::Remove,
            _ => return Err("Unknown Command")
        };

        // get(3) returns `Some(&String)` if index 3 exists, otherwise `None`.
        // cloned() converts `Option<&String>` into `Option<String>`.
        let argument = args.get(3).cloned();

        if todo != "todo" {
            return Err("Invalid command. Use: todo <command> [argument]");
        }

        Ok(Config {
            todo,
            command,
            argument,
        })
    }
}

pub fn run(config: Config, todo_list: &mut TodoList) -> Result<(), &'static str> {
    match config.command {
        Command::Add => {
            let description = config
                .argument
                .ok_or("Provide a description of the task. Use: todo add <description>")?;

            todo_list.create_task(description);

            
        }
        Command::List => todo_list.list_all_tasks(),
        // Command::Done => {},
        // Command::Remove => {}
        _ => return Err("Unknown command"),
    }

    Ok(())
}

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

pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {} - {}", self.id, self.status, self.description)
    }
}

pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }

    fn generate_id(&self) -> u32 {
        self.tasks
            .iter()
            .map(|task| task.id)
            .max()
            .unwrap_or(0)
            + 1
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
        for task in &self.tasks {
            println!("{task}");
        }
    }

    pub fn complete_task(&mut self, task_id: u32) {
        match self.find_task_mut(task_id) {
            Some(task) => task.status = Status::Completed,
            None => eprintln!("Task not found. id: {task_id}"),
        }
    }

    pub fn remove_task(&mut self, task_id: u32) {
        match self.find_task_index(task_id) {
            Some(index) => {
                self.tasks.remove(index);
            }
            None => eprintln!("Task not found. id: {task_id}"),
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
