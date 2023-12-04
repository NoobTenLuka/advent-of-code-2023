use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (winning, having) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let winning_split = winning.split_whitespace();
            having
                .split_whitespace()
                .fold(0.5f32, |acc, have| {
                    if winning_split.clone().contains(&have) {
                        return acc * 2.0;
                    }
                    acc
                })
                .floor() as i32
        })
        .sum::<i32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut next_multipliers = [1i32; 206];

    input
        .lines()
        .enumerate()
        .map(|(num, line)| {
            let mult = next_multipliers[num];
            let (winning, having) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let winning_split = winning.split_whitespace();
            let wins = having
                .split_whitespace()
                .filter(|have| winning_split.clone().contains(have))
                .count();
            
            for i in 1..=wins {
               next_multipliers[num + i] += 1 * mult;
            }

            mult 
        })
        .sum::<i32>()
        .to_string()
}
