mod instruction;

use clap::{App, Arg};
use std::fs;
use std::str::FromStr;

use crate::instruction::{Instruction, Step};

fn main() {
    let file_path = get_args();

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let instructions = parse_instructions(&contents); 
    print_instructions(instructions);
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

fn parse_instructions(contents: &str) -> Vec<Instruction> {
    contents
        .split('\n')
        .filter(|i| !i.is_empty())
        .map(|i| i.trim().split(' ').collect::<Vec<&str>>())
        .map(|i| Instruction {
            state: i[0],
            read: i[1],
            write: i[2],
            step: Step::from_str(i[3]).unwrap(),
            next: i[4],
        })
        .collect::<Vec<Instruction>>()
}

fn print_instructions(instructions: Vec<Instruction>) {
    for ins in instructions {
        println!("{:?}", ins);
    }
}
