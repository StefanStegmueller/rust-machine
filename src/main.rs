mod instruction;
mod machine;
mod parser;

use clap::{App, Arg};
use std::fs;
use std::io;

use crate::instruction::Instruction;
use crate::machine::Machine;
use crate::parser::parse_instructions;

fn main() {
    let file_path = get_args();
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let instructions = parse_instructions(&contents);
    print_seperator();
    print_instructions(&instructions);
    print_seperator();

    println!("Enter initial tape:");
    let mut tape = String::new();
    io::stdin().read_line(&mut tape).unwrap();
    tape = tape.trim().to_string();

    println!("Enter initial state:");
    let mut state = String::new();
    io::stdin().read_line(&mut state).unwrap();
    state = state.trim().to_string();

    let mut machine = Machine::new(state, instructions, tape.chars().collect());

    print_seperator();
    println!("STATE\tTAPE");
    loop {
        machine.print();
        io::stdin().read_line(&mut "".to_string()).unwrap();
        if !machine.next() {
            break;
        }
    }
}

fn get_args() -> String {
    let matches = App::new("rust-machine")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Turing machine interpreter written in rust")
        .arg(
            Arg::with_name("PROGRAM_FILE")
                .required(true)
                .takes_value(true)
                .help("Path to file that contains the turing machine progam"),
        )
        .get_matches();
    let file_path = matches.value_of("PROGRAM_FILE").unwrap();
    file_path.to_string()
}

fn print_seperator() {
    println!("------------------------------------");
}

fn print_instructions(instructions: &Vec<Instruction>) {
    println!("INSTRUCTIONS");
    println!("STATE\tREAD\tWRITE\tSTEP\tNEXT");
    for ins in instructions {
        println!("{}", ins);
    }
}
