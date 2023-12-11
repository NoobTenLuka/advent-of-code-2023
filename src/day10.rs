use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut start = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        start = (x, y);
                    }
                    char
                })
                .collect_vec()
        })
        .collect_vec();

    let mut first_dir = (0, 0);
    let mut second_dir = (0, 0);

    // Field above
    match grid[start.1 - 1][start.0] {
        '|' | '7' | 'F' => first_dir = (start.0, start.1 - 1),
        _ => (),
    };

    // Field below
    match grid[start.1 + 1][start.0] {
        '|' | 'J' | 'L' if first_dir.0 == 0 && first_dir.1 == 0 => {
            first_dir = (start.0, start.1 + 1)
        }
        '|' | 'J' | 'L' => second_dir = (start.0, start.1 + 1),
        _ => (),
    };

    // Field left
    match grid[start.1][start.0 - 1] {
        '-' | 'F' | 'L' if first_dir.0 == 0 && first_dir.1 == 0 => {
            first_dir = (start.0 - 1, start.1)
        }
        '-' | 'F' | 'L' => second_dir = (start.0 - 1, start.1),
        _ => (),
    };

    // Field right
    match grid[start.1][start.0 + 1] {
        '-' | '7' | 'J' if first_dir.0 == 0 && first_dir.1 == 0 => {
            first_dir = (start.0 + 1, start.1)
        }
        '-' | '8' | 'J' => second_dir = (start.0 + 1, start.1),
        _ => (),
    };

    let mut count = 1;
    let mut prev_first = start;
    let mut prev_second = start;
    while first_dir != second_dir {
        let f = update_pos(&grid, first_dir, prev_first);
        prev_first = first_dir;
        first_dir = f;
        let s = update_pos(&grid, second_dir, prev_second);
        prev_second = second_dir;
        second_dir = s;
        count += 1;
    }

    count.to_string()
}

fn update_pos(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    prev_pos: (usize, usize),
) -> (usize, usize) {
    match grid[pos.1][pos.0] {
        '|' => {
            if prev_pos.1 < pos.1 {
                (pos.0, pos.1 + 1)
            } else {
                (pos.0, pos.1 - 1)
            }
        }
        '-' => {
            if prev_pos.0 < pos.0 {
                (pos.0 + 1, pos.1)
            } else {
                (pos.0 - 1, pos.1)
            }
        }
        'L' => {
            if prev_pos.1 < pos.1 {
                (pos.0 + 1, pos.1)
            } else {
                (pos.0, pos.1 - 1)
            }
        }
        'J' => {
            if prev_pos.1 < pos.1 {
                (pos.0 - 1, pos.1)
            } else {
                (pos.0, pos.1 - 1)
            }
        }
        '7' => {
            if prev_pos.0 < pos.0 {
                (pos.0, pos.1 + 1)
            } else {
                (pos.0 - 1, pos.1)
            }
        }
        'F' => {
            if prev_pos.0 > pos.0 {
                (pos.0, pos.1 + 1)
            } else {
                (pos.0 + 1, pos.1)
            }
        }
        _ => unreachable!(),
    }
}

pub fn part2(input: String) -> String {
    let mut start = (0, 0);
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        start = (x, y);
                    }

                    (false, char)
                })
                .collect_vec()
        })
        .collect_vec();

    let mut first_dir = (0, 0);
    let mut second_dir = (0, 0);

    // Field above
    match grid[start.1 - 1][start.0].1 {
        '|' | '7' | 'F' => first_dir = (start.0, start.1 - 1),
        _ => (),
    };

    // Field below
    match grid[start.1 + 1][start.0].1 {
        '|' | 'J' | 'L' if first_dir.0 == 0 && first_dir.1 == 0 => {
            first_dir = (start.0, start.1 + 1)
        }
        '|' | 'J' | 'L' => second_dir = (start.0, start.1 + 1),
        _ => (),
    };

    // Field left
    match grid[start.1][start.0 - 1].1 {
        '-' | 'F' | 'L' if first_dir.0 == 0 && first_dir.1 == 0 => {
            first_dir = (start.0 - 1, start.1)
        }
        '-' | 'F' | 'L' => second_dir = (start.0 - 1, start.1),
        _ => (),
    };

    // Field right
    match grid[start.1][start.0 + 1].1 {
        '-' | '7' | 'J' if first_dir.0 == 0 && first_dir.1 == 0 => {
            first_dir = (start.0 + 1, start.1)
        }
        '-' | '8' | 'J' => second_dir = (start.0 + 1, start.1),
        _ => (),
    };

    let mut prev_first = start;
    let mut prev_second = start;
    grid[start.1][start.0].0 = true;
    grid[first_dir.1][first_dir.0].0 = true;
    grid[second_dir.1][second_dir.0].0 = true;
    while first_dir != second_dir {
        let f = update_pos2(&grid, first_dir, prev_first);
        prev_first = first_dir;
        first_dir = f;
        let s = update_pos2(&grid, second_dir, prev_second);
        prev_second = second_dir;
        second_dir = s;
        
        grid[f.1][f.0].0 = true;
        grid[s.1][s.0].0 = true;
    }

    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(x, elem)| {
                    if elem.0 {
                        return false;
                    }

                    let before = (0..*x)
                        .filter(|x| {
                            let elem = grid[y][*x];

                            if !elem.0 {
                                return false;
                            }

                            match elem.1 {
                                '|' | 'L' | 'J' | 'S' => true,
                                _ => false,
                            }
                        })
                        .count();

                    let after = (*x + 1..grid[0].len())
                        .filter(|x| {
                            let elem = grid[y][*x];

                            if !elem.0 {
                                return false;
                            }

                            match elem.1 {
                                '|' | 'L' | 'J' | 'S' => true,
                                _ => false,
                            }
                        })
                        .count();

                    before % 2 == 1 && after % 2 == 1
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn update_pos2(
    grid: &Vec<Vec<(bool, char)>>,
    pos: (usize, usize),
    prev_pos: (usize, usize),
) -> (usize, usize) {
    match grid[pos.1][pos.0].1 {
        '|' => {
            if prev_pos.1 < pos.1 {
                (pos.0, pos.1 + 1)
            } else {
                (pos.0, pos.1 - 1)
            }
        }
        '-' => {
            if prev_pos.0 < pos.0 {
                (pos.0 + 1, pos.1)
            } else {
                (pos.0 - 1, pos.1)
            }
        }
        'L' => {
            if prev_pos.1 < pos.1 {
                (pos.0 + 1, pos.1)
            } else {
                (pos.0, pos.1 - 1)
            }
        }
        'J' => {
            if prev_pos.1 < pos.1 {
                (pos.0 - 1, pos.1)
            } else {
                (pos.0, pos.1 - 1)
            }
        }
        '7' => {
            if prev_pos.0 < pos.0 {
                (pos.0, pos.1 + 1)
            } else {
                (pos.0 - 1, pos.1)
            }
        }
        'F' => {
            if prev_pos.0 > pos.0 {
                (pos.0, pos.1 + 1)
            } else {
                (pos.0 + 1, pos.1)
            }
        }
        _ => unreachable!(),
    }
}
