use std::{
    env,
    process
};

use todo_list_cli::{
    Config, 
    TodoList
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut todo_list = TodoList::new();

    if let Err(err) = todo_list_cli::run(config, &mut todo_list) {
        eprintln!("Aplication error: {err}");
        process::exit(1);
    }
}
