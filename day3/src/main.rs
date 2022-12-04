use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_lines(path: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

pub fn calculate_priority(letters: &Vec<char>) -> u32 {
    let mut priority: u32 = 0;
    for c in letters {
        if c.is_ascii_lowercase() {
            priority += (*c as u8 - 96) as u32;
        } else if c.is_ascii_uppercase() {
            priority += (*c as u8 - 38) as u32;
        } else {
            panic!("Wrong letter!");
        }
    }
    priority
}

pub fn get_common_items<'a>(mut backpack: String) -> Vec<char> {
    assert!(backpack.len() % 2 == 0);
    let splitted = backpack.split_off(backpack.len() / 2);
    let first_half: HashSet<char> = backpack.chars().collect();
    let second_half: HashSet<char> = splitted.chars().collect();
    let common_items: Vec<char> = first_half.intersection(&second_half).map(|c| *c).collect();
    common_items
}

// Split strings in half and look for common elements.
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
//
pub fn execute_first(path: &str) -> u32 {
    let mut priorities_sum: u32 = 0;

    for line in get_lines(path) {
        match line {
            Ok(line) => {
                let common_items = get_common_items(line);
                let priority = calculate_priority(&common_items);
                priorities_sum += priority;
            },
            _ => (),
        }
    }
    priorities_sum
}

pub fn execute_second(_path: &str) -> u32 {
    0
}

fn main() {
    let first: u32 = execute_first("input");
    let second: u32 = execute_second("input");
    println!("Solution to first problem: {first}");
    println!("Solution to first problem: {second}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_first() {
        assert_eq!(crate::execute_first("input_test"), 157);
    }

    //     #[test]
    //     fn example_second() {
    //         assert_eq!(execute_first("input_test"), 157);
    //     }
}