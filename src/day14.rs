use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let line_count = lines.count() + 1;

    let mut modifiers: Vec<(usize, usize)> = vec![(line_count, 0); first_line.len()];

    input
        .lines()
        .enumerate()
        .map(|(j, line)| {
            line.chars()
                .enumerate()
                .map(|(i, char)| match char {
                    'O' => {
                        let current_mod = modifiers[i];
                        modifiers[i] = (current_mod.0, current_mod.1 + 1);
                        current_mod.0 - current_mod.1
                    }
                    '#' => {
                        modifiers[i] = (line_count - (j + 1), 0);
                        0
                    }
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut input_vec = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut cache: Vec<Vec<Vec<char>>> = vec![];
    while !cache.contains(&input_vec) {
        cache.push(input_vec.clone());
        for y in 0..input_vec.len() {
            for x in 0..input_vec[0].len() {
                if input_vec[y][x] == 'O' {
                    let mut above = y.checked_sub(1).and_then(|res| {
                        input_vec
                            .get(res)
                            .and_then(|inner| (inner[x] == '.').then_some(()))
                    });
                    let mut modifier = 1;
                    while above.is_some() {
                        modifier += 1;
                        above = y.checked_sub(modifier).and_then(|res| {
                            input_vec
                                .get(res)
                                .and_then(|inner| (inner[x] == '.').then_some(()))
                        });
                    }
                    input_vec[y][x] = '.';
                    input_vec[y - (modifier - 1)][x] = 'O';
                }
            }
        }
        for y in 0..input_vec.len() {
            for x in 0..input_vec[0].len() {
                if input_vec[y][x] == 'O' {
                    let mut left = x.checked_sub(1).and_then(|res| {
                        input_vec
                            .get(y)
                            .and_then(|inner| (inner[res] == '.').then_some(()))
                    });
                    let mut modifier = 1;
                    while left.is_some() {
                        modifier += 1;
                        left = x.checked_sub(modifier).and_then(|res| {
                            input_vec
                                .get(y)
                                .and_then(|inner| (inner[res] == '.').then_some(()))
                        });
                    }
                    input_vec[y][x] = '.';
                    input_vec[y][x - (modifier - 1)] = 'O';
                }
            }
        }
        for y in (0..input_vec.len()).rev() {
            for x in 0..input_vec[0].len() {
                if input_vec[y][x] == 'O' {
                    let mut below = y.checked_add(1).and_then(|res| {
                        input_vec
                            .get(res)
                            .and_then(|inner| (inner[x] == '.').then_some(()))
                    });
                    let mut modifier = 1;
                    while below.is_some() {
                        modifier += 1;
                        below = y.checked_add(modifier).and_then(|res| {
                            input_vec
                                .get(res)
                                .and_then(|inner| (inner[x] == '.').then_some(()))
                        });
                    }
                    input_vec[y][x] = '.';
                    input_vec[y + (modifier - 1)][x] = 'O';
                }
            }
        }
        for y in 0..input_vec.len() {
            for x in (0..input_vec[0].len()).rev() {
                if input_vec[y][x] == 'O' {
                    let mut right = x.checked_add(1).and_then(|res| {
                        input_vec[y]
                            .get(res)
                            .and_then(|inner| (*inner == '.').then_some(()))
                    });
                    let mut modifier = 1;
                    while right.is_some() {
                        modifier += 1;
                        right = x.checked_add(modifier).and_then(|res| {
                            input_vec[y]
                                .get(res)
                                .and_then(|inner| (*inner == '.').then_some(()))
                        });
                    }
                    input_vec[y][x] = '.';
                    input_vec[y][x + (modifier - 1)] = 'O';
                }
            }
        }
    }
    let start_index = cache.iter().position(|x| *x == input_vec).unwrap();

    let index = start_index + ((1000000000 - start_index) % (cache.len() - start_index));

    let sum = cache[index]
        .iter()
        .enumerate()
        .map(|(j, row)| {
            row.iter()
                .map(|char| {
                    if *char == 'O' {
                        return input_vec.len() - j;
                    }
                    0
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string();

    sum
}
