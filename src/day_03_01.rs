#[derive(Debug)]
enum Schematic {
    Number {
        row: usize,
        col: usize,
        len: usize,
        value: u32,
    },
    Symbol {
        row: usize,
        col: usize,
        value: char,
    },
    Whitespace {
        row: usize,
        col: usize,
        value: char,
    },
}

pub fn solve(input: &str) -> u32 {
    let mut rows: Vec<Vec<Schematic>> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut row_vec = Vec::new();
        let mut col = 0;
        let chars: Vec<char> = line.chars().collect();
        while col < line.len() {
            match chars[col] {
                d if d.is_digit(10) => {
                    let digits: String = chars
                        .iter()
                        .skip(col)
                        .take_while(|c| c.is_digit(10))
                        .collect();

                    let len = digits.len();

                    row_vec.push(Schematic::Number {
                        row,
                        col,
                        len,
                        value: digits
                            .parse::<u32>()
                            .expect(format!("number parse error: {}", digits).as_str()),
                    });
                    col += len;
                }
                '.' => {
                    col += 1;
                }
                symbol => {
                    row_vec.push(Schematic::Symbol {
                        row,
                        col,
                        value: symbol,
                    });
                    col += 1;
                }
            }
        }
        rows.push(row_vec);
    }

    let numbers = rows
        .iter()
        .flatten()
        .filter(|x| match x {
            Schematic::Number {
                row,
                col,
                len,
                value,
            } => {
                rows.iter()
                    .flatten()
                    .filter(|s| {
                        if let Schematic::Symbol {
                            row: sym_row,
                            col: sym_col,
                            value: _,
                        } = s
                        {
                            match (*row, *col, *col + *len - 1, *sym_row, *sym_col) {
                                (0, num_col, num_last_col, sym_row, sym_col) => {
                                    sym_row <= 1
                                        && sym_col <= num_last_col + 1
                                        && (num_col == 0 || sym_col >= num_col - 1)
                                }
                                (num_row, 0, num_last_col, sym_row, sym_col) => {
                                    sym_col <= num_last_col + 1
                                        && sym_row >= num_row - 1
                                        && sym_row <= num_row + 1
                                }
                                (num_row, num_col, num_last_col, sym_row, sym_col) => {
                                    sym_row >= num_row - 1
                                        && sym_row <= num_row + 1
                                        && sym_col >= num_col - 1
                                        && sym_col <= num_last_col + 1
                                }
                            }
                        } else {
                            false
                        }
                    })
                    .count()
                    > 0
            }
            _ => false,
        })
        .map(|x| {
            if let Schematic::Number {
                row: _,
                col: _,
                len: _,
                value,
            } = x
            {
                *value
            } else {
                0 as u32
            }
        })
        .sum::<u32>();

    numbers
}
