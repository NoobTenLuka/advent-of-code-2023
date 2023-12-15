mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

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
        7 => {
            if part1 {
                day07::part1(input)
            } else {
                day07::part2(input)
            }
        }
        8 => {
            if part1 {
                day08::part1(input)
            } else {
                day08::part2(input)
            }
        }
        9 => {
            if part1 {
                day09::part1(input)
            } else {
                day09::part2(input)
            }
        }
        10 => {
            if part1 {
                day10::part1(input)
            } else {
                day10::part2(input)
            }
        }
        11 => {
            if part1 {
                day11::part1(input)
            } else {
                day11::part2(input)
            }
        }
        12 => {
            if part1 {
                day12::part1(input)
            } else {
                day12::part2(input)
            }
        }
        13 => {
            if part1 {
                day13::part1(input)
            } else {
                day13::part2(input)
            }
        }
        14 => {
            if part1 {
                day14::part1(input)
            } else {
                day14::part2(input)
            }
        }
        15 => {
            if part1 {
                day15::part1(input)
            } else {
                day15::part2(input)
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
