use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if let Ok(file) = File::open("input") {
        let lines = BufReader::new(file).lines();
        let mut elfs: Vec<u32> = vec![];
        let mut counted_calories: u32 = 0;

        // Sum up calories
        for l in lines {
            match l {
                Ok(line) => {
                    if line.is_empty() {
                        elfs.push(counted_calories);
                        counted_calories = 0;
                    } else {
                        let n: u32 = line.parse().unwrap();
                        counted_calories += n;
                    }
                },
                _ => (),
            }
        }

        elfs.sort();
        let bigger = elfs[elfs.len() - 1];
        println!("maximum: {:?}", bigger);

        let last_three: u32 = elfs[elfs.len() - 3..].iter().sum();
        println!("sum of last three: {:?}", last_three);
    }
}
