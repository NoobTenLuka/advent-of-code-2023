use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut map = input
        .lines()
        .map(|line| line.chars().map(|char| (char, false)).collect_vec())
        .collect_vec();

    CachedTraverse::new().traverse(&mut map, (-1, 0), (1, 0));

    map.iter()
        .map(|row| row.iter().filter(|(_, b)| *b).count())
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().map(|char| (char, false)).collect_vec())
        .collect_vec();

    [
        (-1, 0),
        (map[0].len() as isize, 0),
        (0, -1),
        (0, map.len() as isize),
    ]
    .iter()
    .map(|(start_x, start_y)| {
        if *start_x == 0 {
            (0..map[0].len())
                .map(|x| {
                    let mut cloned_map = map.clone();
                    let direction = if *start_y == -1 { (0, 1) } else { (0, -1) };
                    CachedTraverse::new().traverse(
                        &mut cloned_map,
                        (x as isize, *start_y),
                        direction,
                    );

                    cloned_map
                        .iter()
                        .map(|row| row.iter().filter(|(_, b)| *b).count())
                        .sum::<usize>()
                })
                .max()
                .unwrap()
        } else {
            (0..map.len())
                .map(|y| {
                    let mut cloned_map = map.clone();
                    let direction = if *start_x == -1 { (1, 0) } else { (-1, 0) };
                    CachedTraverse::new().traverse(
                        &mut cloned_map,
                        (*start_x, y as isize),
                        direction,
                    );

                    cloned_map
                        .iter()
                        .map(|row| row.iter().filter(|(_, b)| *b).count())
                        .sum::<usize>()
                })
                .max()
                .unwrap()
        }
    })
    .max()
    .unwrap()
    .to_string()
}

struct CachedTraverse {
    cache: HashSet<(isize, isize, isize, isize)>,
}

impl CachedTraverse {
    fn new() -> Self {
        Self {
            cache: HashSet::new(),
        }
    }
    fn traverse(
        &mut self,
        map: &mut [Vec<(char, bool)>],
        current_pos: (isize, isize),
        current_direction: (isize, isize),
    ) {
        let mut next_node = None;
        let mut next_direction = (0, 0);
        let mut next_second_direction = None;

        let current_path = (
            current_pos.0,
            current_pos.1,
            current_direction.0,
            current_direction.1,
        );
        if self.cache.contains(&current_path) {
            return;
        }

        let mut steps = 1;
        let mut new_field = false;
        loop {
            let next_y = match current_pos.1.checked_add(current_direction.1 * steps) {
                Some(y) => y as usize,
                None => break,
            };
            let next_x = match current_pos.0.checked_add(current_direction.0 * steps) {
                Some(x) => x as usize,
                None => break,
            };
            if map
                .get_mut(next_y)
                .and_then(|row| {
                    row.get_mut(next_x).map(|(char, visited)| {
                        steps += 1;
                        *visited = true;
                        new_field = true;

                        next_node = Some((next_x as isize, next_y as isize));
                        match char {
                            '/' => {
                                next_direction = match current_direction {
                                    (1, _) => (0, -1),
                                    (-1, _) => (0, 1),
                                    (_, 1) => (-1, 0),
                                    (_, -1) => (1, 0),
                                    _ => unreachable!(),
                                };
                            }
                            '\\' => {
                                next_direction = match current_direction {
                                    (1, _) => (0, 1),
                                    (-1, _) => (0, -1),
                                    (_, 1) => (1, 0),
                                    (_, -1) => (-1, 0),
                                    _ => unreachable!(),
                                };
                            }
                            '|' => {
                                if current_direction.1 == 0 {
                                    next_second_direction = Some((0, -1));
                                    next_direction = (0, 1);
                                } else {
                                    new_field = false;
                                };
                            }
                            '-' => {
                                if current_direction.0 == 0 {
                                    next_second_direction = Some((-1, 0));
                                    next_direction = (1, 0);
                                } else {
                                    new_field = false;
                                };
                            }
                            _ => new_field = false,
                        }
                    })
                })
                .is_none()
                || new_field
            {
                break;
            }
        }

        if !new_field {
            return;
        }

        match next_node {
            Some(next_node) => {
                self.cache.insert(current_path);
                match next_second_direction {
                    Some(nsd) => self.traverse(map, next_node, nsd),
                    None => (),
                }
                self.traverse(map, next_node, next_direction);
            }
            None => return,
        }
    }
}
