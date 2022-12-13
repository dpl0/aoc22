#![feature(iter_next_chunk)]

use std::fs::read_to_string;

type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug, Clone)]
struct StackMachine(Vec<Stack>);

impl StackMachine {
    // Logic for solving first problem
    pub fn apply_instruction_cratemover_9000(&mut self, instruction: &MoveInstruction) {
        // Start counting from 0
        let origin = instruction.origin - 1;
        let to = instruction.to - 1;
        for _ in 0..instruction.amount {
            if let Some(crate_letter) = self.0[origin].pop() {
                self.0[to].push(crate_letter);
            }
        }
    }

    // Logic for solving second problem
    pub fn apply_instruction_cratemover_9001(&mut self, instruction: &MoveInstruction) {
        // Start counting from 0
        let origin = instruction.origin - 1;
        let to = instruction.to - 1;

        let mut crate_buffer: Vec<Crate> = vec![];
        for _ in 0..instruction.amount {
            if let Some(crate_letter) = self.0[origin].pop() {
                crate_buffer.insert(0, crate_letter);
            }
        }
        for letter in crate_buffer {
            self.0[to].push(letter);
        }
    }

    pub fn get_top_crates(self) -> String {
        self.0.iter().filter_map(|stack| stack.last().copied()).collect()
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
        let number_of_stacks = (number_line.len() + 1) / 4;

        // Populate the stack machine first with empty vectors
        let mut stack_machine = StackMachine(vec![vec![]; number_of_stacks]);

        for line in lines {
            let mut chars = line.chars();
            for stack_index in 0..number_of_stacks {
                let Ok([_, crate_id, _]) = chars.next_chunk() else {
                    panic!("Badly formatted stack. Check the MoveCrater documentation.");
                };

                if crate_id != ' ' {
                    stack_machine.0[stack_index].push(crate_id);
                }

                // Ignore space at the end
                chars.next();
            }
        }
        stack_machine
    }
}

#[derive(Debug)]
pub struct MoveInstruction {
    amount: usize,
    origin: usize,
    to: usize,
}

impl MoveInstruction {
    /// Parses the actual instruction from the line
    pub fn new(line: &str) -> Self {
        let mut splitted = line.split_whitespace();

        // move A from B to C
        let Ok(["move", amount, "from", origin, "to", to]) = splitted.next_chunk() else {
            panic!("Badly formatted instruction.");
        };

        let (Ok(amount), Ok(origin), Ok(to)) = (amount.parse(), origin.parse(), to.parse()) else {
            panic!("Couldn't parse fields from instruction");
        };

        MoveInstruction { amount, origin, to }
    }
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

    let instructions: Vec<MoveInstruction> = lines_instructions
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(MoveInstruction::new)
        .collect();

    for instruction in instructions {
        stack_first_problem.apply_instruction_cratemover_9000(&instruction);
        stack_second_problem.apply_instruction_cratemover_9001(&instruction);
    }

    println!("Answer for first problem: {}", stack_first_problem.get_top_crates());
    println!("Answer for second problem: {}", stack_second_problem.get_top_crates());
}
