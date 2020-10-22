use crate::instruction::{Instruction, State, Step};

pub struct Machine {
    tape: Vec<char>,
    state: State,
    head: i32,
    program: Vec<Instruction>,
}

impl Machine {
    pub fn new(initial_state: State, program: Vec<Instruction>, tape: Vec<char>) -> Self {
        Self {
            tape,
            state: initial_state,
            head: 0,
            program,
        }
    }

    pub fn next(&mut self) -> bool {
        for i in &self.program {
            if i.state == self.state && i.read == *self.read() {
                self.tape[self.head as usize] = i.write;
                match i.step {
                    Step::L => {
                        self.head = self.head - 1;
                        if self.head == -1 {
                            self.tape.insert(0, '0');
                            self.head = 0;
                        }
                    }
                    Step::R => {
                        self.head = self.head + 1;
                        if self.head as usize >= self.tape.len() {
                            self.tape.push('0');
                        }
                    }
                }
                self.state = i.next.clone();
                return true;
            }
        }
        false
    }

    pub fn print(&mut self) {
        let to_string = |vc: &Vec<char>| vc.iter().collect::<String>();
        let mut head: Vec<char> = vec![' '; self.tape.len()];
        head.insert(self.head as usize, '^');
        head.pop();
        println!("({})\t{}", self.state, to_string(&self.tape));
        println!("\t{}", to_string(&head));
    }

    fn read(&self) -> &char {
        &self.tape[self.head as usize]
    }
}
