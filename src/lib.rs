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

// struct Task 
// {
//     id: i32,
//     description: String,
//     status: String,
// }
// impl Task
// {
//     // return an task
//     fn create_task(description: String, status: String) {}

//     fn list_all_tasks() {}

//     fn done_task(task_id: i32) {}

//     fn remove_task(task_id: i32) {}

//     fn search_task(task_id: i32) {}

// }
