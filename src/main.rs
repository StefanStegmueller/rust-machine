mod instruction;
mod machine;

use clap::{App, Arg};
use std::fs;
use std::io;
use std::str::FromStr;

use crate::instruction::{Instruction, Step};
use crate::machine::Machine;

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
    println!("START");
    while machine.next() {
        machine.print_tape();
        let _ = io::stdin().read_line(&mut "".to_string()).unwrap();
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

            let r: Vec<char> = i[READ].chars().collect();
            let w: Vec<char> = i[WRITE].chars().collect();

            if r.len() > 1 || w.len() > 1 {
                err(
                    c,
                    "A read or write token is expected to have a length of 1.",
                )
            }

            Instruction {
                state: i[STATE].to_string(),
                read: r[0],
                write: w[0],
                step: Step::from_str(i[STEP]).unwrap_or_else(|_| {
                    err(c, "Token for step direction has to be either L or R.");
                    Step::L
                }),
                next: i[NEXT].to_string(),
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
