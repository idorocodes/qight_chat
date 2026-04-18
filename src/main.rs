use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

pub mod logic;
pub use logic::*;

fn main() {
    loop {
        println!("Welcome to qight chat");
        println!("Lets chat on qight!");
        println!("Use q to quit and close the chat");
        print!("qightchat > ");
        stdout().flush().unwrap();

        let mut input = String::new();

        stdin().read_line(&mut input).expect("failed to read line");

        let command = input.trim();

        if command.is_empty() {
            println!("no input");
            continue;
        }

        if command == "q" {
            println!("Closing qight chat!");
            break;
        }

        let mut child = Command::new(command).spawn().unwrap();

        child.wait().unwrap();
    }
}
