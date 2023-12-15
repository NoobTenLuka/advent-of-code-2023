use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .split("\n\n")
        .map(|block| {
            let block_map = block
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec();

            // Horizontal mirrors
            for y in 0..block_map.len() - 1 {
                let mut modifier = 0;
                loop {
                    if y.checked_sub(modifier).is_none() || y + 1 + modifier >= block_map.len() {
                        // match found
                        return (y + 1) * 100;
                    }

                    if block_map[y - modifier] == block_map[y + 1 + modifier] {
                        // could match, expand
                        modifier += 1;
                        continue;
                    }

                    break;
                }
            }

            // Vertical mirrors
            for x in 0..block_map[0].len() - 1 {
                let mut modifier = 0;
                'outer: loop {
                    if x.checked_sub(modifier).is_none() || x + 1 + modifier >= block_map[0].len() {
                        // match found
                        return x + 1;
                    }
                    for item in &block_map {
                        if item[x - modifier] != item[x + 1 + modifier] {
                            // didn't match
                            break 'outer;
                        }
                    }
                    modifier += 1;
                }
            }

            0
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .split("\n\n")
        .map(|block| {
            let block_map = block
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec();

            // Horizontal mirrors
            for y in 0..block_map.len() - 1 {
                let mut modifier = 0;
                let mut smudge_found = false;
                'outer: loop {
                    if y.checked_sub(modifier).is_none() || y + 1 + modifier >= block_map.len() {
                        // match found
                        if smudge_found {
                            return (y + 1) * 100;
                        }
                        break 'outer;
                    }

                    for x in 0..block_map[0].len() {
                        if block_map[y - modifier][x] != block_map[y + 1 + modifier][x] {
                            if smudge_found {
                                break 'outer;
                            }

                            smudge_found = true;
                        }
                    }
                    modifier += 1;
                }
            }

            // Vertical mirrors
            for x in 0..block_map[0].len() - 1 {
                let mut modifier = 0;
                let mut smudge_found = false;
                'outer: loop {
                    if x.checked_sub(modifier).is_none() || x + 1 + modifier >= block_map[0].len() {
                        // match found
                        if smudge_found {
                            return x + 1;
                        }
                        break 'outer;
                    }
                    for item in &block_map {
                        if item[x - modifier] != item[x + 1 + modifier] {
                            if smudge_found {
                                // didn't match
                                break 'outer;
                            }

                            smudge_found = true;
                        }
                    }
                    modifier += 1;
                }
            }

            0
        })
        .sum::<usize>()
        .to_string()
}
