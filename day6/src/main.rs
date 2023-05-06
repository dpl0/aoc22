use std::collections::HashSet;
use std::fs::read_to_string;

pub fn detect_signal(window_size: usize, data: &str) -> Option<usize> {
    data.as_bytes()
        .windows(window_size)
        .position(|window| {
            // Generate a HashSet for every window, not ideal for memory but it works.
            window.iter().collect::<HashSet<_>>().len() == window_size
        })
        .map(|index| index + window_size)
}

fn main() {
    let input = read_to_string("input").expect("File not found");
    let data: &str = input.split('\n').collect::<Vec<&str>>()[0];

    let result = detect_signal(4, data);
    println!("Problem 1: {:?}", result);

    let result = detect_signal(14, data);
    println!("Problem 2: {:?}", result);
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::detect_signal;

    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    pub fn detect_for_four(index: usize, data: &str) {
        assert_eq!(Some(index), detect_signal(4, data));
    }

    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    pub fn detect_for_fourteen(index: usize, data: &str) {
        assert_eq!(Some(index), detect_signal(14, data));
    }
}
