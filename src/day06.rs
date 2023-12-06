use itertools::Itertools;

pub fn part1(input: String) -> String {
    let (times, distances) = input
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    (0..times.len()).map(|i| {
        let mut j = times[i] / 2;
        let mut correct_count = 0;
        loop {
            let travel_time = times[i] - j;
            let distance = travel_time * j;

            if distance <= distances[i] {
                break;
            };

            correct_count += 1;
            j -= 1;
        }
        correct_count *= 2;
        if times[i] % 2 == 0 { correct_count -= 1 }
        correct_count
    }).product::<i64>().to_string()
}

pub fn part2(input: String) -> String {
    let (time, distance_to_beat) = input
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .split_whitespace()
                .join("")
                .parse::<i64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

        let mut j = time / 2;
        let mut correct_count = 0;
        loop {
            let travel_time = time - j;
            let distance = travel_time * j;

            if distance <= distance_to_beat {
                break;
            };

            correct_count += 1;
            j -= 1;
        }
        correct_count *= 2;
        if time % 2 == 0 { correct_count -= 1 }
        correct_count.to_string()
}
