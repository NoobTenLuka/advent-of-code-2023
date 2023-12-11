use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let mut derivations = Vec::new();
            let first_derivation = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec();
            derivations.push(first_derivation);
            let mut index = 0; 

            loop {
                let new_derivation = derivations[index] 
                    .iter()
                    .tuple_windows()
                    .map(|(curr, next)| {
                        next - curr
                    })
                    .collect_vec();

                derivations.push(new_derivation);
                index += 1;

                if derivations[index].first().unwrap() == derivations[index].last().unwrap() {
                    break;
                }

            }
            let mut next_step = 0;
            for derivation in derivations.iter().rev() {
                next_step = derivation.last().unwrap() + next_step; 
            }

            next_step 
        })
        .sum::<i32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let mut derivations = Vec::new();
            let first_derivation = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec();
            derivations.push(first_derivation);
            let mut index = 0; 

            loop {
                let new_derivation = derivations[index] 
                    .iter()
                    .tuple_windows()
                    .map(|(curr, next)| {
                        next - curr
                    })
                    .collect_vec();

                derivations.push(new_derivation);
                index += 1;

                if derivations[index].first().unwrap() == derivations[index].last().unwrap() {
                    break;
                }

            }
            let mut next_step = 0;
            for derivation in derivations.iter().rev() {
                next_step = derivation.first().unwrap() - next_step;
            }

            next_step 
        })
        .sum::<i32>()
        .to_string()
}
