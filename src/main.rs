use std::{
    env,
    process,
};

use todo_list_cli::Config;

fn main() 
{
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = todo_list_cli::run(config) {
        eprintln!("Aplication error: {err}");
        process::exit(1);
    }
}

