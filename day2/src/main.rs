use std::fs::File;
use std::io::{BufRead, BufReader};

/// Possible results from any clash
#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

/// Possible shapes in the game
#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

use Outcome::*;
use Shape::*;

///
/// Variant of the clash for the first problem
///
struct ClashFirst {
    elf: Shape,
    me: Shape,
}

impl ClashFirst {
    pub fn outcome(self) -> u32 {
        let outcome = match (self.elf, self.me) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Win,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Loss,
            (a, b) if a == b => Draw,
            // Should not happen
            _ => Draw,
        };

        // Calculate score depending on outcome
        outcome.score() + self.me.score()
    }
}

///
/// Variant of the clash for the second problem
///
struct ClashSecond {
    elf: Shape,
    outcome: Outcome,
}

impl ClashSecond {
    pub fn outcome(self) -> u32 {
        let my_hand = match (self.elf, self.outcome) {
            (Rock, Loss) | (Paper, Win) => Scissors,
            (Rock, Win) | (Scissors, Loss) => Paper,
            (Paper, Loss) | (Scissors, Win) => Rock,
            (_, Draw) => self.elf,
        };

        self.outcome.score() + my_hand.score()
    }
}

fn main() {
    if let Ok(file) = File::open("input") {
        let lines = BufReader::new(file).lines();

        let mut score_first = 0;
        let mut score_second = 0;

        // Part 1
        for l in lines {
            match l {
                Ok(line) => {
                    let hands: Vec<&str> = line.split(" ").collect();
                    let elf = match hands[0] {
                        "A" => Rock,
                        "B" => Paper,
                        "C" => Scissors,
                        _ => panic!("That elf is cheating!"),
                    };

                    let me = match hands[1] {
                        "X" => Rock,
                        "Y" => Paper,
                        "Z" => Scissors,
                        _ => panic!("I'm cheating!??"),
                    };

                    let outcome = match hands[1] {
                        "X" => Loss,
                        "Y" => Draw,
                        "Z" => Win,
                        _ => panic!("I'm cheating!??"),
                    };

                    score_first += ClashFirst { elf, me }.outcome();
                    score_second += ClashSecond { elf, outcome }.outcome();
                }
                _ => (),
            }
        }
        println!("Problem 1: My final score is {score_first}");
        println!("Problem 2: My final score is {score_second}");
    }
}
