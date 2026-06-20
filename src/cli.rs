/// Command-line interface layer.
///
/// Responsible for parsing command-line arguments,
/// validating user input, and building the application configuration.

pub enum Command {
    Add,
    List,
    Done,
    Undone,
    Remove,
}

pub struct Config {
    pub command: Command,
    pub argument: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Use: todo <command> [\"argument\"]");
        }


        if args[1].to_lowercase() != "todo" {
            return Err("Invalid command. Use: todo <command> [\"argument\"]");
        }

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


        Ok(Config {
            command,
            argument,
        })
    }
}

