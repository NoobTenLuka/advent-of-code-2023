pub fn part1(input: String) -> String {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut x_expansions = vec![true; input.split_once('\n').unwrap().0.len()];

    let mut y_expansions = 0;
    for (y, line) in input.lines().enumerate() {
        let mut current_expands = true;
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                points.push((x, y + y_expansions));
                x_expansions[x] = false;
                current_expands = false;
            }
        }
        if current_expands {
            y_expansions += 1;
        }
    }

    for (i, _) in x_expansions.iter().enumerate().filter(|(_, x)| **x).rev() {
        for point in points.iter_mut().filter(|(x, _)| x > &i) {
            point.0 += 1;
        }
    }

    let mut sum = 0;
    for (i, point) in points.iter().enumerate() {
        for other in points.iter().skip(i + 1) {
            let diff = point.0.abs_diff(other.0) + point.1.abs_diff(other.1);
            sum += diff;
        }
    }

    sum.to_string()
}

pub fn part2(input: String) -> String {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut x_expansions = vec![true; input.split_once('\n').unwrap().0.len()];

    let mut y_expansions = 0;
    for (y, line) in input.lines().enumerate() {
        let mut current_expands = true;
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                points.push((x, y + (y_expansions * (1000000 - 1))));
                x_expansions[x] = false;
                current_expands = false;
            }
        }
        if current_expands {
            y_expansions += 1;
        }
    }

    for (i, _) in x_expansions.iter().enumerate().filter(|(_, x)| **x).rev() {
        for point in points.iter_mut().filter(|(x, _)| x > &i) {
            point.0 += 1000000 - 1;
        }
    }

    let mut sum = 0;
    for (i, point) in points.iter().enumerate() {
        for other in points.iter().skip(i + 1) {
            let diff = point.0.abs_diff(other.0) + point.1.abs_diff(other.1);
            sum += diff;
        }
    }

    sum.to_string()
}
