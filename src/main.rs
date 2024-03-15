//importing in execute! macro
#[macro_use]
extern crate crossterm;

use clap::{Parser, Subcommand};

mod command;
mod term;

#[derive(Parser)]
#[command(name = "ref")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// search for a reference
    Search { text: Option<String> },

    /// find and execute a reference
    Exec(command::exec::Exec),

    /// find and edit a reference
    Edit { text: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // example of inline definition
        Commands::Search { text } => match text {
            Some(text) => {
                println!("begin search with matches to {:?}", text);
                term::handle_events().unwrap();
            },
            None => println!("begin search with top commands showing.."),
        },

        // example of a command module definition
        Commands::Exec(cmd) => {
            command::exec::run(&cmd);
        }

        // catch all for un-implemented commands
        _ => println!("TODO: command not implemented yet"),
    }
}
