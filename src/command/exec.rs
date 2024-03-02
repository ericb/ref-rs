use clap::Parser;

#[derive(Debug, Parser)]
pub struct Exec {
    text: Option<String>,
}

pub fn run(cmd: &Exec) {
    match &cmd.text {
        Some(text) => println!("run exec with {:?}", text),
        None => println!("run exec with default")
    }
}
