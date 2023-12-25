use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

enum Outcome {
    Rejected,
    Approved,
    Redirected(String),
}

type Part = (usize, usize, usize, usize);

pub fn part1(input: String) -> String {
    let (instructions, parts) = input.split_once("\n\n").unwrap();

    let instruction_map: HashMap<String, Box<dyn Fn(Part) -> Outcome>> = instructions
        .lines()
        .map(|instruction| {
            let (name, rest) = instruction.split_once("{").unwrap();

            (
                name.to_owned(),
                Box::new(move |(x, m, a, s)| {
                    let mini_instructions = rest.split(",");
                    for mini_instruction in mini_instructions {
                        let (condition, res) = match mini_instruction.split_once(":") {
                            Some(val) => val,
                            None => return str_to_outcome(mini_instruction.trim_end_matches("}")),
                        };

                        let value = match &condition[0..1] {
                            "x" => x,
                            "m" => m,
                            "a" => a,
                            "s" => s,
                            &_ => 0,
                        };

                        let num = &condition[2..condition.len()].parse::<usize>().unwrap();
                        let matched = match &condition[1..2] {
                            "<" => value < *num,
                            ">" => value > *num,
                            &_ => unreachable!(),
                        };

                        if matched {
                            return str_to_outcome(res);
                        }
                    }
                    Outcome::Rejected
                }) as Box<dyn Fn(Part) -> Outcome>,
            )
        })
        .collect();

    let num_regex = Regex::new("\\d+").unwrap();

    parts
        .lines()
        .filter_map(|part| {
            let part: Part = num_regex
                .find_iter(part)
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            let mut next_val = "in".to_owned();

            loop {
                let res = instruction_map[&next_val](part);

                match res {
                    Outcome::Approved => return Some(part.0 + part.1 + part.2 + part.3),
                    Outcome::Rejected => return None,
                    Outcome::Redirected(val) => next_val = val,
                }
            }
        })
        .sum::<usize>()
        .to_string()
}

fn str_to_outcome(input: &str) -> Outcome {
    match input {
        "A" => Outcome::Approved,
        "R" => Outcome::Rejected,
        val => Outcome::Redirected(val.to_owned()),
    }
}

pub fn part2(input: String) -> String {
    let (instructions, _) = input.split_once("\n\n").unwrap();

    let instruction_map: HashMap<String, Vec<&str>> = instructions
        .lines()
        .map(|instruction| {
            let (name, rest) = instruction.split_once("{").unwrap();

            let mini_instructions = rest.split(",");
            (name.to_owned(), mini_instructions.collect_vec())
        })
        .collect();

    recurse(
        (1, 4000),
        (1, 4000),
        (1, 4000),
        (1, 4000),
        "in".to_owned(),
        0,
        &instruction_map,
    )
    .to_string()
}

fn recurse(
    mut x: (usize, usize),
    mut m: (usize, usize),
    mut a: (usize, usize),
    mut s: (usize, usize),
    rule_name: String,
    rule_index: usize,
    instruction_map: &HashMap<String, Vec<&str>>,
) -> usize {
    let mini_instruction = &instruction_map[&rule_name][rule_index];

    let (condition, res) = match mini_instruction.split_once(":") {
        Some(val) => val,
        None => {
            return match str_to_outcome(mini_instruction.trim_end_matches("}")) {
                Outcome::Approved => (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1),
                Outcome::Rejected => 0,
                Outcome::Redirected(val) => recurse(x, m, a, s, val, 0, instruction_map),
            }
        }
    };

    let mut second_x = x;
    let mut second_m = m;
    let mut second_a = a;
    let mut second_s = s;

    let second_value;
    let value = match &condition[0..1] {
        "x" => {
            second_value = &mut second_x;
            &mut x
        }
        "m" => {
            second_value = &mut second_m;
            &mut m
        }
        "a" => {
            second_value = &mut second_a;
            &mut a
        }
        "s" => {
            second_value = &mut second_s;
            &mut s
        }
        &_ => panic!(),
    };

    let num = &condition[2..condition.len()].parse::<usize>().unwrap();
    match &condition[1..2] {
        "<" => {
            *value = (value.0, num - 1);
            *second_value = (*num, second_value.1);
        }
        ">" => {
            *value = (num + 1, value.1);
            *second_value = (second_value.0, *num);
        }
        &_ => unreachable!(),
    };

    let first = match str_to_outcome(res) {
        Outcome::Approved => (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1),
        Outcome::Rejected => 0,
        Outcome::Redirected(val) => recurse(x, m, a, s, val, 0, instruction_map),
    };

    first
        + recurse(
            second_x,
            second_m,
            second_a,
            second_s,
            rule_name,
            rule_index + 1,
            instruction_map,
        )
}
