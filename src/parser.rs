use std::str::FromStr;

use crate::instruction::{Instruction, Step};

pub fn parse_instructions(contents: &str) -> Vec<Instruction> {
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
