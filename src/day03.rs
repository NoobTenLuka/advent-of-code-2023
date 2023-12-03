use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut sum: i32 = 0;
    input
        .lines()
        .enumerate()
        .tuple_windows::<(_, _, _)>()
        .for_each(|lines| {
            lines.1 .1.chars().enumerate().for_each(|(i, char)| {
                if char != '.' && !char.is_ascii_digit() {
                    // Here we have a symbol
                    let mut before = lines.0 .1.chars();
                    let mut current = lines.1 .1.chars();
                    let mut after = lines.2 .1.chars();
                    let mut last_low = 0;
                    if before
                        .nth(i.saturating_sub(1))
                        .is_some_and(|a| a.is_ascii_digit())
                    {
                        let (low, num) = get_num(lines.0 .1, i.saturating_sub(1));
                        
                        sum += num;
                        last_low = low;
                    }
                    if before.next().is_some_and(|a| a.is_ascii_digit()) {
                        let (low, num) = get_num(lines.0 .1, i);
                        if low != last_low {
                            
                            sum += num;
                        }
                        last_low = low;
                    }
                    if before.next().is_some_and(|a| a.is_ascii_digit()) {
                        let (low, num) = get_num(lines.0 .1, i + 1);
                        if low != last_low {
                            
                            sum += num;
                        }
                    }
                    if current
                        .nth(i.saturating_sub(1))
                        .is_some_and(|a| a.is_ascii_digit())
                    {
                        let (_, num) = get_num(lines.1 .1, i.saturating_sub(1));
                        
                        sum += num;
                    }
                    if current.nth(1).is_some_and(|a| a.is_ascii_digit()) {
                        let (_, num) = get_num(lines.1 .1, i + 1);
                        
                        sum += num;
                    }
                    last_low = 0;
                    if after
                        .nth(i.saturating_sub(1))
                        .is_some_and(|a| a.is_ascii_digit())
                    {
                        let (low, num) = get_num(lines.2 .1, i.saturating_sub(1));
                        
                        sum += num;
                        last_low = low;
                    }
                    if after.next().is_some_and(|a| a.is_ascii_digit()) {
                        let (low, num) = get_num(lines.2 .1, i);
                        if low != last_low {
                            
                            sum += num;
                        }
                        last_low = low;
                    }
                    if after.next().is_some_and(|a| a.is_ascii_digit()) {
                        let (low, num) = get_num(lines.2 .1, i + 1);
                        if low != last_low {
                            
                            sum += num;
                        }
                    }
                }
            });
        });
    sum.to_string()
}

pub fn part2(input: String) -> String {
    let mut sum: i32 = 0;
    input
        .lines()
        .enumerate()
        .tuple_windows::<(_, _, _)>()
        .for_each(|lines| {
            lines.1 .1.chars().enumerate().for_each(|(i, char)| {
                if char == '*' {
                    // Here we have a symbol
                    let mut before = lines.0 .1.chars();
                    let mut current = lines.1 .1.chars();
                    let mut after = lines.2 .1.chars();
                    let mut last_low = 0;
                    let mut gear_ratio = 1;
                    let mut part_number_count = 0;
                    if before
                        .nth(i.saturating_sub(1))
                        .is_some_and(|a| a.is_ascii_digit())
                    {
                        let (low, num) = get_num(lines.0 .1, i.saturating_sub(1));
                        gear_ratio *= num;
                        part_number_count += 1;
                        last_low = low;
                    }
                    if before.next().is_some_and(|a| a.is_ascii_digit()) {
                        let (low, num) = get_num(lines.0 .1, i);
                        if low != last_low {
                            gear_ratio *= num;
                            part_number_count += 1;
                        }
                        last_low = low;
                    }
                    if before.next().is_some_and(|a| a.is_ascii_digit()) {
                        let (low, num) = get_num(lines.0 .1, i + 1);
                        if low != last_low {
                            gear_ratio *= num;
                            part_number_count += 1;
                        }
                    }
                    if current
                        .nth(i.saturating_sub(1))
                        .is_some_and(|a| a.is_ascii_digit())
                    {
                        let (_, num) = get_num(lines.1 .1, i.saturating_sub(1));
                        gear_ratio *= num;
                        part_number_count += 1;
                    }
                    if current.nth(1).is_some_and(|a| a.is_ascii_digit()) {
                        let (_, num) = get_num(lines.1 .1, i + 1);
                        gear_ratio *= num;
                        part_number_count += 1;
                    }
                    last_low = 0;
                    if after
                        .nth(i.saturating_sub(1))
                        .is_some_and(|a| a.is_ascii_digit())
                    {
                        let (low, num) = get_num(lines.2 .1, i.saturating_sub(1));
                        gear_ratio *= num;
                        part_number_count += 1;
                        last_low = low;
                    }
                    if after.next().is_some_and(|a| a.is_ascii_digit()) {
                        let (low, num) = get_num(lines.2 .1, i);
                        if low != last_low {
                            gear_ratio *= num;
                            part_number_count += 1;
                        }
                        last_low = low;
                    }
                    if after.next().is_some_and(|a| a.is_ascii_digit()) {
                        let (low, num) = get_num(lines.2 .1, i + 1);
                        if low != last_low {
                            gear_ratio *= num;
                            part_number_count += 1;
                        }
                    }
                    if part_number_count == 2 {
                        sum += gear_ratio;
                    }
                }
            });
        });
    sum.to_string()
}

fn get_num(line: &str, known_point: usize) -> (usize, i32) {
    let mut low = known_point;
    let mut high = known_point;

    while low != 0
        && line
            .as_bytes()
            .get(low - 1)
            .is_some_and(|x| x.is_ascii_digit())
    {
        low -= 1;
    }

    while line
        .as_bytes()
        .get(high + 1)
        .is_some_and(|x| x.is_ascii_digit())
    {
        high += 1;
    }

    (low, line[low..=high].parse::<i32>().unwrap())
}
