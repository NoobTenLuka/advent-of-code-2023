pub fn part1(input: String) -> String {
    input.trim().split(',').map(hash).sum::<usize>().to_string()
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
}

pub fn part2(input: String) -> String {
    let mut hash_map: [Vec<(&str, usize)>; 256] = std::array::from_fn(|_| Vec::new());
    input.trim().split(',').for_each(|op| {
        let is_dash_operation = op.ends_with('-');
        let (label, operand) = if is_dash_operation {
            op.split_once('-').unwrap()
        } else {
            op.split_once('=').unwrap()
        };

        let hash = hash(label);
        if is_dash_operation {
            hash_map[hash]
                .iter()
                .position(|(l, _)| l == &label)
                .map(|i| hash_map[hash].remove(i));
        } else if hash_map[hash]
            .iter()
            .position(|(l, _)| l == &label)
            .map(|i| hash_map[hash][i].1 = operand.parse::<usize>().unwrap())
            .is_none()
        {
            hash_map[hash].push((label, operand.parse::<usize>().unwrap()));
        };
    });

    hash_map
        .iter()
        .enumerate()
        .map(|(box_index, inner)| {
            inner
                .iter()
                .enumerate()
                .map(|(slot, (_, focal))| (box_index + 1) * (slot + 1) * focal)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}
