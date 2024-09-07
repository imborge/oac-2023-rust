use std::collections::VecDeque;

pub fn first_and_last_digit(line: &str) -> (u32, u32) {
    let digits: VecDeque<_> = line
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    (*digits.front().unwrap(), *digits.back().unwrap())
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (a, b) = first_and_last_digit(line);
            10 * a + b
        })
        .sum()
}
