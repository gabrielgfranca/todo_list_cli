use std::fmt::{self, write};

pub struct Config
{
    pub todo: String,
    pub command: String,
    pub argument: Option<String>,
}

impl Config
{
    pub fn build(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3 {
            return Err("Not enough arguments. Use: todo <command> [argument]");
        }

        let todo = args[1].to_lowercase();
        let command = args[2].to_lowercase();
        
        // get(3) returns `Some(&String)` if index 3 exists, otherwise `None`.
        // cloned() converts `Option<&String>` into `Option<String>`.
        let argument = args.get(3).cloned(); 

        if todo != "todo" {
            return Err("Invalid command. Use: todo <command> [argument]");
        }

        Ok(Config {
            todo,
            command,
            argument
        })
    }
}

pub fn run(config: Config) -> Result<(), &'static str>
{
    match &config.argument {
        Some(description) => {
            println!("Adding task {description}");
        }

        None => {
            return Err("Provide a description of the task. Use: todo add <description>")
        }
    }

    Ok(())
}

enum Status {
    Pending,
    Completed,
}

impl fmt::Display for Status
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

pub struct Task
{
    pub id: u32,
    pub description: String,
    pub status: Status,
}

impl Task
{
    pub fn info(&self)
    {
        let id = self.id;
        let desc = &self.description;
        let status = self.status.to_string();

        println!("
        Id: {id},
        Status: {status},
        Description: {desc}
        ");
    }
}

pub struct TodoList
{
    tasks: Vec<Task>
}

impl TodoList
{
    pub fn new() -> Self
    {
        Self {
            tasks: Vec::new()
        }
    }

    fn generate_id(&self) -> u32
    {
        self.tasks
                .iter()
                .map(|task| task.id)
                .max()
                .unwrap_or(0)
                + 1
    }

    pub fn create_task(&mut self, description: String)
    {
        let task = Task {
            id: self.generate_id(),
            description,
            status: Status::Pending,
        };

        self.tasks.push(task)
    }

    pub fn list_all_tasks(&self)
    {
        for task in &self.tasks {
            task.info();
        }
    }
}
//     fn list_all_tasks() {}

//     fn done_task(task_id: i32) {}

//     fn remove_task(task_id: i32) {}

//     fn search_task(task_id: i32) {}
