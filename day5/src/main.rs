use std::fs::read_to_string;

use regex::Regex;

type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug, Clone)]
struct StackMachine(Vec<Stack>);

impl StackMachine {
    // Logic for solving first problem
    pub fn apply_instruction_cratemover_9000(&mut self, instruction: &MoveInstruction) {
        // Start counting from 0
        let origin = instruction.origin - 1;
        let destination = instruction.destination - 1;
        for _ in 0..instruction.amount {
            if let Some(crate_letter) = self.0[origin].pop() {
                self.0[destination].push(crate_letter);
            }
        }
    }

    // Logic for solving second problem
    pub fn apply_instruction_cratemover_9001(&mut self, instruction: &MoveInstruction) {
        // Start counting from 0
        let origin = instruction.origin - 1;
        let destination = instruction.destination - 1;

        let mut crate_buffer: Vec<Crate> = vec![];
        for _ in 0..instruction.amount {
            if let Some(crate_letter) = self.0[origin].pop() {
                crate_buffer.insert(0, crate_letter);
            }
        }
        for letter in crate_buffer {
            self.0[destination].push(letter);
        }
    }

    pub fn get_top_crates(&self) -> Vec<Crate> {
        let mut retval: Vec<Crate> = vec![];
        for s in &self.0 {
            if let Some(c) = s.last() {
                retval.push(*c);
            }
        }
        retval
    }

    fn get_stack_number(number_line: &str) -> u32 {
        //  1   2   3
        let contains_numbers = Regex::new(r"( /d  ?)*").unwrap();
        assert!(contains_numbers.is_match(number_line));
        // Get last number of line
        let Ok(number_of_stacks) = number_line.split_whitespace()
            .last()
            .expect("Couldn't get last field of the numbers line")
            .parse::<u32>() else {
            panic!("Couldn't parse last element of stack number");
        };
        number_of_stacks
    }

    pub fn new(stack_repr: Vec<&str>) -> Self {
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3

        // Reverse to build stack by just pushing.
        let mut lines = stack_repr.iter().rev();

        // Handle the line with the numbers
        let Some(number_line) = lines.next() else { panic!("Couldn't get the stack machine's first line");};
        let number_of_stacks = Self::get_stack_number(number_line);

        // Populate the stack machine first with empty vectors
        let mut stack_machine = StackMachine(vec![]);
        for _ in 0..number_of_stacks {
            stack_machine.0.push(vec![]);
        }

        let mut stack_index = 0;
        for line in lines {
            for (i, c) in line.chars().enumerate().step_by(4) {
                if c == '[' {
                    let crate_letter: Crate = line.as_bytes()[i + 1] as Crate;
                    stack_machine.0[stack_index].push(crate_letter);
                }
                stack_index += 1;
            }
            // Start all over
            stack_index = 0;
        }
        stack_machine
    }
}

#[derive(Debug)]
pub struct MoveInstruction {
    amount: usize,
    origin: usize,
    destination: usize,
}

impl MoveInstruction {
    /// Parses the actual instruction from the line
    pub fn new(line: &str) -> Self {
        // move A from B to C
        let mut splitted = line.split_whitespace();

        // Those 1 are Dark Magic, honestly.
        // It's because `nth` partially consumes the iterator.
        let (Some(amount), Some(origin), Some(destination)) = (splitted.nth(1), splitted.nth(1), splitted.nth(1)) else {
            panic!("Couldn't get field from instruction");
        };

        let (Ok(amount), Ok(origin), Ok(destination)) = (amount.parse(), origin.parse(), destination.parse()) else {
            panic!("Couldn't parse fields from instruction");
        };

        MoveInstruction {
            amount,
            origin,
            destination,
        }
    }
}

pub fn parse_instructions(lines: Vec<&str>) -> Vec<MoveInstruction> {
    let instruction_regex = Regex::new(r"^move \d+ from \d+ to \d+$").unwrap();
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            assert!(instruction_regex.is_match(l));
            MoveInstruction::new(l)
        })
        .collect()
}

fn main() {
    let input = read_to_string("input").expect("File not found");
    let lines: Vec<&str> = input.split('\n').collect();

    // From start until first empty line we can find the initial state.
    // From first empty line until EOF or another empty line, we'll find the instructions;
    let mut lines_splitter = lines.split(|l| l.is_empty());

    let lines_stack: Vec<&str> = lines_splitter.next().expect("Couldn't read the stack state").into();
    let lines_instructions: Vec<&str> = lines_splitter.next().expect("Couldn't read the instructions").into();

    let mut stack_first_problem: StackMachine = StackMachine::new(lines_stack);
    let mut stack_second_problem: StackMachine = stack_first_problem.clone();

    let instructions: Vec<MoveInstruction> = parse_instructions(lines_instructions);
    for instruction in instructions {
        stack_first_problem.apply_instruction_cratemover_9000(&instruction);
        stack_second_problem.apply_instruction_cratemover_9001(&instruction);
    }

    println!("Answer for first problem: {:?}", stack_first_problem.get_top_crates());
    println!("Answer for second problem: {:?}", stack_second_problem.get_top_crates());
}
