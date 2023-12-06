mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::{env, fs};

fn main() {
    let day: i8 = env::args()
        .nth(1)
        .expect("Not a valid day!")
        .parse()
        .expect("Not a valid day!");

    let part1 = match env::args().nth(2).expect("Not a valid part!").as_str() {
        "1" => true,
        "2" => false,
        _ => panic!("Not a valid part!"),
    };

    let test_input = match env::args().nth(3) {
        Some(val) => val,
        None => "".to_owned(),
    };

    let input = fs::read_to_string(format!("inputs/{:02}{}-input.txt", day, test_input))
        .expect("Day is invalid");

    let output = match day {
        1 => {
            if part1 {
                day01::part1(input)
            } else {
                day01::part2(input)
            }
        }
        2 => {
            if part1 {
                day02::part1(input)
            } else {
                day02::part2(input)
            }
        }
        3 => {
            if part1 {
                day03::part1(input)
            } else {
                day03::part2(input)
            }
        }
        4 => {
            if part1 {
                day04::part1(input)
            } else {
                day04::part2(input)
            }
        }
        5 => {
            if part1 {
                day05::part1(input)
            } else {
                day05::part2(input)
            }
        }
        6 => {
            if part1 {
                day06::part1(input)
            } else {
                day06::part2(input)
            }
        }
        _ => panic!("Either invalid or not implemented"),
    };

    println!(
        "Day {:02} part {} resulted in: {}",
        day,
        if part1 { 1 } else { 2 },
        output
    )
}
