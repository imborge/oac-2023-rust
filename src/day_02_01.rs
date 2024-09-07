#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub hands_shown: Vec<Hand>,
}

#[derive(Debug)]
pub struct Hand {
    pub n_red: i32,
    pub n_green: i32,
    pub n_blue: i32,
}

pub fn parse_game(line: &str) -> Game {
    let (game_id_part, game_part) = line.split_once(": ").unwrap();
    let game_id = game_id_part
        .strip_prefix("Game ")
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let hands = game_part
        .split("; ")
        .map(|hand| {
            let mut n_red = 0;
            let mut n_green = 0;
            let mut n_blue = 0;

            for ball_str in hand.split(", ") {
                let (count, color) = ball_str.split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();
                match color {
                    "red" => n_red = count,
                    "green" => n_green = count,
                    "blue" => n_blue = count,
                    _ => panic!("invalid color {}", color),
                }
            }

            Hand {
                n_red,
                n_green,
                n_blue,
            }
        })
        .collect();

    Game {
        id: game_id,
        hands_shown: hands,
    }
}

pub fn solve((r, g, b): (i32, i32, i32), input: &str) -> i32 {
    let games = input.lines().map(parse_game);

    let mut valid_ids = Vec::new();
    for game in games {
        let valid = game
            .hands_shown
            .iter()
            .all(|h| h.n_red <= r && h.n_green <= g && h.n_blue <= b);

        if valid {
            valid_ids.push(game.id);
        }
    }

    valid_ids.iter().sum()
}
