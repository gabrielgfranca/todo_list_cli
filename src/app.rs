
use crate::{
    cli::cli::{
        Config,
        Command
    },
    
    todo_list::todo_list::TodoList
};

pub fn run(config: Config) -> Result<(), &'static str> {
    let mut todo_list = TodoList::load();

    match config.command {
        Command::Add => {
            let description = config
                .argument
                .ok_or("Provide a description of the task. Use: todo add <\"description\">")?;
            
            todo_list.create_task(description);

            println!("Task created successfully");
        }
        
        Command::List => todo_list.list_all_tasks(),
        
        Command::Done => {
            let task_id = parse_task_id(config.argument)?;

            todo_list.done_task(task_id)?;

            println!("Task marked as completed");
        },

        Command::Undone => {
            let task_id = parse_task_id(config.argument)?;

            todo_list.undone_task(task_id)?;

            println!("Task marked as pending");
        }
        
        Command::Remove => {
            let task_id = parse_task_id(config.argument)?;

            todo_list.remove_task(task_id)?;

            println!("Task removed successfully");
        }
    }

    todo_list.save()
        .map_err(|_|"Failed to save task")?;

    Ok(())
}

fn parse_task_id(argument: Option<String>) -> Result<u32, &'static str> {
    let argument = argument.ok_or("Provide task Id")?;
        
    argument
        .parse::<u32>()
        .map_err(|_| "Invalid task id")
}