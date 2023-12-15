use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut operations = input.split("\n\n");
    let mut seeds = operations
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    operations.for_each(|operation| {
        let mut dest_map = Vec::new();
        operation.split('\n').skip(1).for_each(|op_line| {
            let (dest, source_start, range) = op_line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .next_tuple()
                .unwrap();
            let source_end = source_start + range;
            let diff = source_start - dest;

            for i in (0..seeds.len()).rev() {
                let seed = seeds[i];
                if seed >= source_start && seed < source_end {
                    dest_map.push(seed - diff);
                    seeds.remove(i);
                }
            }
        });
        seeds.append(&mut dest_map);
    });

    seeds.iter().min().unwrap().to_string()
}

// Please use --release thx
pub fn part2(input: String) -> String {
    let mut operations = input.split("\n\n");
    let seeds = operations
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .tuples::<(_, _)>()
        .collect_vec();

    let op_vec = operations.collect_vec();
    let new_operations = op_vec.iter().rev();

    for x in 0..i64::MAX {
        let mut current_dest = x;
        'outer: for operation in new_operations.clone() {
            for op_line in operation.split('\n').skip(1) {
                let (dest_start, source_start, range) = op_line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .next_tuple()
                    .unwrap();
                let dest_end = dest_start + range;
                if current_dest >= dest_start && current_dest < dest_end {
                    let diff = source_start - dest_start;
                    current_dest += diff;
                    continue 'outer;
                }
            }
        }
        for (start, len) in &seeds {
            let end = start + len;
            if current_dest >= *start && current_dest < end {
                return x.to_string();
            }
        }
    }

    0.to_string()
}

// Unfinished, but may result in a better runtime once finished
#[allow(dead_code)]
pub fn part2_idea(input: String) -> String {
    let mut operations = input.split("\n\n");
    let mut seeds = operations
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .tuples::<(_, _)>()
        .map(|(start, len)| (start, start + len))
        .collect_vec();

    operations.for_each(|operation| {
        let mut new_ranges = Vec::new();
        operation.split('\n').skip(1).for_each(|op_line| {
            let (dest, source_start, range) = op_line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .next_tuple()
                .unwrap();
            let source_end = source_start + range;
            let diff = source_start - dest;
            for (start, end) in &seeds {
                if source_start >= *start && source_end <= *end {
                    // Equal or smaller range
                    new_ranges.push((source_start - diff, source_end - diff));
                } else if source_start < *end && source_start >= *start {
                    println!("{start} {end} {source_start} {source_end} {diff}");
                    new_ranges.push((source_start - diff, end - diff));
                } else if source_end > *start && source_start < *start {
                    new_ranges.push((start - diff, source_end - diff));
                }
            }
        });
        if !new_ranges.is_empty() {
            seeds = new_ranges;
        }
    });

    seeds.iter().map(|(s, _)| s).min().unwrap().to_string()
}
