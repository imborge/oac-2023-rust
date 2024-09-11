mod day_01_01;
mod day_01_02;
mod day_02_01;
mod day_02_02;
mod day_03_01;

fn get_input(fp: &str) -> String {
    std::fs::read_to_string(format!("data/{}.txt", fp)).unwrap()
}

fn main() {
    // Day 1
    println!("Day 01 Part 01: {}", day_01_01::solve(&get_input("day_01")));
    println!("Day 01 Part 02: {}", day_01_02::solve(&get_input("day_01")));

    // Day 2
    println!(
        "Day 02 Part 01: {}",
        day_02_01::solve((12, 13, 14), &get_input("day_02"))
    );
    println!("Day 02 Part 01: {}", day_02_02::solve(&get_input("day_02")));

    // Day 3
    println!("Day 03 Part 01: {}", day_03_01::solve(&get_input("day_03")));
}
