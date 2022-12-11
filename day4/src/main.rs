use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    // A-B
    pub fn new(range: &str) -> Self {
        let splitted: Vec<&str> = range.split('-').collect();
        assert_eq!(splitted.len(), 2);

        if let (Ok(start), Ok(end)) = (splitted[0].parse::<u32>(), splitted[1].parse::<u32>()) {
            Range { start, end }
        } else {
            panic!("Badly formatted file!");
        }
    }

    pub fn is_contained(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn is_overlapping_with(&self, other: &Range) -> bool {
        // Start is in set
        (self.start >= other.start && self.start <= other.end) || (self.end >= other.start && self.end <= other.end)
    }
}

fn main() {
    let input = read_to_string("input").expect("File not found");
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    // A-B,C-D
    let contained: Vec<(Range, Range)> = lines
        .iter()
        .map(|l| l.split(',').collect::<Vec<&str>>())
        .map(|pair| (Range::new(pair[0]), Range::new(pair[1])))
        .filter(|(first, second)| first.is_contained(second) || second.is_contained(first))
        .collect();

    let overlapping: Vec<(Range, Range)> = lines
        .iter()
        .map(|l| l.split(',').collect::<Vec<&str>>())
        .map(|pair| (Range::new(pair[0]), Range::new(pair[1])))
        .filter(|(first, second)| first.is_overlapping_with(second) || second.is_overlapping_with(first))
        .collect();

    println!("Answer for first problem: {}", contained.len());
    println!("Answer for second problem: {}", overlapping.len());
}
