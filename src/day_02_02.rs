use std::cmp::max;

use crate::day_02_01::{parse_game, Game, Hand};

fn max_cubes_by_color(hands: Vec<Hand>) -> (i32, i32, i32) {
    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;

    for hand in hands {
        max_r = max(max_r, hand.n_red);
        max_g = max(max_g, hand.n_green);
        max_b = max(max_b, hand.n_blue);
    }

    (max_r, max_g, max_b)
}

pub fn solve(input: &str) -> i32 {
    let games = input.lines().map(parse_game);

    games
        .map(|g| {
            let (r, g, b) = max_cubes_by_color(g.hands_shown);
            r * g * b
        })
        .sum()
}
