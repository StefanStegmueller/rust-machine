use std::fmt;
use std::str::FromStr;

pub type State = String;
pub type Symbol = char;

#[derive(Debug)]
pub enum Step {
    L,
    R,
    N,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(input: &str) -> Result<Step, Self::Err> {
        match input {
            "L" => Ok(Step::L),
            "R" => Ok(Step::R),
            "N" => Ok(Step::N),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Step::L => write!(f, "L"),
            Step::R => write!(f, "R"),
            Step::N => write!(f, "N"),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub state: State,
    pub read: Symbol,
    pub write: Symbol,
    pub step: Step,
    pub next: State,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}",
            self.state, self.read, self.write, self.step, self.next
        )
    }
}
