/// Application entry point.
///
/// Responsible for:
/// - Collecting command-line arguments.
/// - Building the application configuration.
/// - Starting the application workflow.
/// - Handling and reporting errors.

use std::{
    env,
    process
};

use todo_list_cli::{
    cli::Config,
    app,   
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = app::run(config) {
        eprintln!("Aplication error: {err}");
        process::exit(1);
    }
}
