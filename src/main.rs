mod day_01_01;
mod day_01_02;

fn get_input(fp: &str) -> String {
    std::fs::read_to_string(format!("data/{}.txt", fp)).unwrap()
}

fn main() {
    println!("Day 01 Part 01: {}", day_01_01::solve(&get_input("day_01")));

    println!("Day 01 Part 02: {}", day_01_02::solve(&get_input("day_01")));
}
