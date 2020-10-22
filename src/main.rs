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
    let (file_path, _break) = collect_args();
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let instructions = parse_instructions(&contents);
    print_seperator();
    print_instructions(&instructions);
    print_seperator();

    let (tape, state, blank, head_index) = ask_params();
    let mut machine = Machine::new(state, blank, head_index, instructions, tape);

    print_seperator();
    println!("STATE\tTAPE");
    loop {
        machine.print();
        if _break {
            io::stdin().read_line(&mut "".to_string()).unwrap();
        }
        if !machine.next() {
            break;
        }
    }
}

fn collect_args() -> (String, bool) {
    let matches = App::new("rust-machine")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Turing machine interpreter written in rust")
        .arg(
            Arg::with_name("PROGRAM_FILE")
                .required(true)
                .takes_value(true)
                .help("Path to file that contains the Turing machine program"),
        )
        .arg(
            Arg::with_name("break")
                .short("b")
                .long("break")
                .takes_value(false)
                .help("Flag to break execution after each step"),
        )
        .get_matches();
    let file_path = matches.value_of("PROGRAM_FILE").unwrap();
    let _break: bool = matches.is_present("break");
    (file_path.to_string(), _break)
}

fn ask_params() -> (Vec<char>, String, char, i32) {
    println!("Enter initial tape:");
    let mut tape = String::new();
    io::stdin().read_line(&mut tape).unwrap();
    let v_tape: Vec<char> = tape.trim().to_string().chars().collect();

    println!("Enter initial state:");
    let mut state = String::new();
    io::stdin().read_line(&mut state).unwrap();
    state = state.trim().to_string();

    println!("Enter blank symbol:");
    let mut blank = String::new();
    io::stdin().read_line(&mut blank).unwrap();
    let c_blank: char = blank.trim().to_string().chars().collect::<Vec<char>>()[0];
    
    println!("Enter initial head position:");
    let mut head_index = String::new();
    io::stdin().read_line(&mut head_index).unwrap();
    let i_head_index: i32 = head_index.trim().to_string().parse::<i32>().unwrap();

    (v_tape, state, c_blank, i_head_index)
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
