use std::str::FromStr;

type State<'a> = &'a str;
type Symbol<'a> = &'a str;

#[derive(Debug)]
pub enum Step {
    L,
    R,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(input: &str) -> Result<Step, Self::Err> {
        match input {
            "L" => Ok(Step::L),
            "R" => Ok(Step::R),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Instruction<'a> {
    pub state: State<'a>,
    pub read: Symbol<'a>,
    pub write: Symbol<'a>,
    pub step: Step,
    pub next: State<'a>,
}
