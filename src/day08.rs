use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;
use regex::Regex;

pub fn part1(input: String) -> String {
    let regex = Regex::new(r"[A-Z]+").unwrap();
    let map: HashMap<&str, usize> = input
        .lines()
        .skip(2)
        .enumerate()
        .map(|(i, line)| {
            let key = regex.find(line).unwrap().as_str();

            (key, i)
        })
        .collect();

    let machine = input
        .lines()
        .skip(2)
        .map(|line| {
            let (from_str, to_str) = regex
                .find_iter(line)
                .skip(1)
                .map(|x| x.as_str())
                .collect_tuple()
                .unwrap();

            (map.get(from_str).unwrap(), map.get(to_str).unwrap())
        })
        .collect_vec();

    let mut count = 0u64;
    let mut state = map.get("AAA").unwrap();
    let end_state = map.get("ZZZ").unwrap();
    for lr in input.lines().next().unwrap().chars().cycle() {
        if state == end_state {
            break;
        }

        match lr {
            'L' => state = machine[*state].0,
            'R' => state = machine[*state].1,
            _ => unreachable!(),
        }

        count += 1;
    }

    count.to_string()
}

pub fn part2(input: String) -> String {
    let regex = Regex::new(r"[A-Z]+").unwrap();
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let map: HashMap<&str, usize> = input
        .lines()
        .skip(2)
        .enumerate()
        .map(|(i, line)| {
            let key = regex.find(line).unwrap().as_str();

            if key.ends_with('A') {
                starts.push(i);
            }
            if key.ends_with('Z') {
                ends.push(i);
            }

            (key, i)
        })
        .collect();

    let machine = input
        .lines()
        .skip(2)
        .map(|line| {
            let (from_str, to_str) = regex
                .find_iter(line)
                .skip(1)
                .map(|x| x.as_str())
                .collect_tuple()
                .unwrap();

            (map.get(from_str).unwrap(), map.get(to_str).unwrap())
        })
        .collect_vec();

    starts
        .iter()
        .map(|s| {
            let mut count = 0u64;
            let mut state = s;
            for lr in input.lines().next().unwrap().chars().cycle() {
                if ends.contains(state) {
                    break;
                }

                match lr {
                    'L' => state = machine[*state].0,
                    'R' => state = machine[*state].1,
                    _ => unreachable!(),
                }

                count += 1;
            }

            count
        })
        .reduce(|acc, x| x.lcm(&acc))
        .unwrap()
        .to_string()
}
