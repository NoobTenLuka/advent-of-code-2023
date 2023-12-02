pub fn part1(input: String) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut only_nums = line.chars().filter_map(|c| c.to_digit(10));
            let first = only_nums.next().unwrap();
            let last = only_nums.last().unwrap_or(first);
            10 * first + last
        })
        .sum();

    sum.to_string()
}

pub fn part2(input: String) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let modified_line = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");
            let mut only_nums = modified_line.chars().filter_map(|c| c.to_digit(10));
            let first = only_nums.next().unwrap();
            let last = only_nums.last().unwrap_or(first);
            10 * first + last
        })
        .sum();

    sum.to_string()
}
