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

pub fn get_common_items(mut backpack: String) -> Vec<char> {
    assert!(backpack.len() % 2 == 0);
    let splitted = backpack.split_off(backpack.len() / 2);
    let first_half: HashSet<char> = backpack.chars().collect();
    let second_half: HashSet<char> = splitted.chars().collect();
    let common_items: Vec<char> = first_half.intersection(&second_half).copied().collect();
    common_items
}

// Split strings in half and look for common elements.
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
pub fn execute_first(path: &str) -> u32 {
    let mut priorities_sum: u32 = 0;

    for line in get_lines(path).flatten() {
        let common_items = get_common_items(line);
        let priority = calculate_priority(&common_items);
        priorities_sum += priority;
    }
    priorities_sum
}

// Find one common element for packs of 3 lines.
// Calculate the priority of that pack and calculate the total.
pub fn execute_second(path: &str) -> u32 {
    let mut priorities_sum: u32 = 0;

    let mut lines = get_lines(path);
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next()) {
        let first: HashSet<char> = first.expect("Couldn't get string").chars().collect();
        let second: HashSet<char> = second.expect("Couldn't get string").chars().collect();
        let third: HashSet<char> = third.expect("Couldn't get string").chars().collect();
        let common_items: HashSet<char> = first.intersection(&second).copied().collect();
        let common_items: Vec<char> = third.intersection(&common_items).copied().collect();
        let priority = calculate_priority(&common_items);
        priorities_sum += priority;
    }
    priorities_sum
}

fn main() {
    let first: u32 = execute_first("input");
    let second: u32 = execute_second("input");
    println!("Solution to first problem: {first}");
    println!("Solution to first problem: {second}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn test_line(line: Option<Result<String, std::io::Error>>, common_items_given: Vec<char>, priority_given: u32) {
        let line = line.unwrap().unwrap();
        let common = get_common_items(line);
        assert_eq!(common, common_items_given);
        let priority = calculate_priority(&common);
        assert_eq!(priority, priority_given);
    }

    #[test]
    fn example_first() {
        // 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s)
        let mut lines = get_lines("input_test");
        test_line(lines.next(), vec!['p'], 16);
        test_line(lines.next(), vec!['L'], 38);
        test_line(lines.next(), vec!['P'], 42);
        test_line(lines.next(), vec!['v'], 22);
        test_line(lines.next(), vec!['t'], 20);
        test_line(lines.next(), vec!['s'], 19);
        drop(lines);

        // The sum of these is 157.
        assert_eq!(execute_first("input_test"), 157);
    }

    #[test]
    fn example_second() {
        assert_eq!(execute_second("input_test"), 70);
    }
}
