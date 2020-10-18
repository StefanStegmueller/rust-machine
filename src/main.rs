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
    let errmsg_std = |c| format!("\nParsing error in line {}:\n", c + 1);
    let err = |c, m| panic!("{}{}", errmsg_std(c), m);

    const STATE: usize = 0;
    const READ: usize = 1;
    const WRITE: usize = 2;
    const STEP: usize = 3;
    const NEXT: usize = 4;

    let instructions = contents
        .split('\n')
        .filter(|i| !i.is_empty())
        .map(|i| i.trim().split(' ').collect::<Vec<&str>>())
        .enumerate()
        .map(|(c, i)| {
            if i.len() != 5 {
                err(c, "An instructions is expected to have 5 tokens.");
            }

            Instruction {
                state: i[STATE],
                read: i[READ],
                write: i[WRITE],
                step: Step::from_str(i[STEP]).unwrap_or_else(|_| {
                    err(c, "Token for step direction has to be either L or R.")
                }),
                next: i[NEXT],
            }
        })
        .collect::<Vec<Instruction>>();

    // check for ambiguous instructions
    let mut unique_instructions: Vec<&Instruction> = vec![];
    for (c, i) in instructions.iter().enumerate() {
        if !unique_instructions
            .iter()
            .any(|&ui| ui.state == i.state && ui.read == i.read)
        {
            unique_instructions.push(i);
        } else {
            err(c, "This instruction is ambiguous. Another instructions includes the same state and read token.");
        }
    }

    instructions
}

fn print_instructions(instructions: Vec<Instruction>) {
    for ins in instructions {
        println!("{:?}", ins);
    }
}
