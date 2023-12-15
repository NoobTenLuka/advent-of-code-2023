use std::cmp::Ordering;

use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .sorted_by(|(hand1, _), (hand2, _)| sort_hands(hand2, hand1))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid.parse::<usize>().unwrap())
        .sum::<usize>()
        .to_string()
}

fn sort_hands(hand1: &str, hand2: &str) -> Ordering {
    let mut kinds1 = [0; 13];
    let mut diff_1 = 0;
    let mut kinds2 = [0; 13];
    let mut diff_2 = 0;

    for c in hand1.chars() {
        let index = char_to_power(&c) as usize;

        if kinds1[index] == 0 {
            diff_1 += 1;
        }
        kinds1[index] += 1;
    }
    for c in hand2.chars() {
        let index = char_to_power(&c) as usize;

        if kinds2[index] == 0 {
            diff_2 += 1;
        }
        kinds2[index] += 1;
    }

    if diff_1 > diff_2 {
        return Ordering::Greater;
    }

    if diff_2 > diff_1 {
        return Ordering::Less;
    }

    if diff_1 == 2 || diff_1 == 3 {
        let max_1 = kinds1.iter().max().unwrap();
        let max_2 = kinds2.iter().max().unwrap();
        let ordering = max_2.cmp(max_1);
        if ordering.is_ne() {
            return ordering;
        }
    }

    for (c1, c2) in hand1.chars().zip(hand2.chars()) {
        let p1 = char_to_power(&c1);
        let p2 = char_to_power(&c2);
        let ordering = p2.cmp(&p1);
        if ordering.is_ne() {
            return ordering;
        }
    }

    Ordering::Equal
}

fn char_to_power(c: &char) -> u32 {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        x => x.to_digit(10).unwrap() - 2,
    }
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .sorted_by(|(hand1, _), (hand2, _)| sort_hands2(hand2, hand1))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid.parse::<usize>().unwrap())
        .sum::<usize>()
        .to_string()
}

fn sort_hands2(hand1: &str, hand2: &str) -> Ordering {
    let mut kinds1 = [0; 13];
    let mut diff_1 = 0;
    let mut kinds2 = [0; 13];
    let mut diff_2 = 0;

    for c in hand1.chars() {
        let index = char_to_power2(&c) as usize;

        if index != 0 && kinds1[index] == 0 {
            diff_1 += 1;
        }

        kinds1[index] += 1;
    }

    if diff_1 == 0 {
        diff_1 = 1
    }

    for c in hand2.chars() {
        let index = char_to_power2(&c) as usize;

        if index != 0 && kinds2[index] == 0 {
            diff_2 += 1;
        }
        kinds2[index] += 1;
    }

    if diff_2 == 0 {
        diff_2 = 1
    }

    if diff_1 > diff_2 {
        return Ordering::Greater;
    }

    if diff_2 > diff_1 {
        return Ordering::Less;
    }

    if diff_1 == 2 || diff_1 == 3 {
        let max_1 = kinds1.iter().skip(1).max().unwrap() + kinds1[0];
        let max_2 = kinds2.iter().skip(1).max().unwrap() + kinds2[0];
        let ordering = max_2.cmp(&max_1);
        if ordering.is_ne() {
            return ordering;
        }
    }

    for (c1, c2) in hand1.chars().zip(hand2.chars()) {
        let p1 = char_to_power2(&c1);
        let p2 = char_to_power2(&c2);
        let ordering = p2.cmp(&p1);
        if ordering.is_ne() {
            return ordering;
        }
    }

    Ordering::Equal
}

fn char_to_power2(c: &char) -> u32 {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        'J' => 0,
        x => x.to_digit(10).unwrap() - 1,
    }
}
