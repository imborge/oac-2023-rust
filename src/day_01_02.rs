pub fn first_and_last_digit(s: &str) -> (u32, u32) {
    //     .replace("one", "1")
    //     .replace("two", "2")
    //     .replace("three", "3")
    //     .replace("four", "4")
    //     .replace("five", "5")
    //     .replace("six", "6")
    //     .replace("seven", "7")
    //     .replace("eight", "8")
    //     .replace("nine", "9")

    let patterns = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut digits = Vec::new();

    for (p, d) in patterns {
        if let Some(i) = s.find(p) {
            digits.push((i, d));
        }

        if let Some(i) = s.rfind(p) {
            digits.push((i, d))
        }
    }

    digits.sort_by(|(ai, _), (bi, _)| ai.cmp(bi));

    let (_, first_digit) = digits.first().unwrap();
    let (_, last_digit) = digits.last().unwrap();

    (*first_digit, *last_digit)
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (a, b) = first_and_last_digit(&line);

            10 * a + b
        })
        .sum()
}
