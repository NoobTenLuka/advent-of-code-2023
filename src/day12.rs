use std::collections::HashMap;

use itertools::Itertools;

/*
 *  Day 12 was a pain in the ass for me. Struggled to came up with a good solutions thats not just
 *  entirely brute-force and ended up with this after watching
 *  https://www.youtube.com/watch?v=g3Ms5e7Jdqo and understanding the solution. I tried to "copy"
 *  his solution from python to rust. Not proud of this one :/ but I am smarter now... hopefully
 */

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (parts, nums_str) = line.split_once(" ").unwrap();
            let nums = nums_str
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();
            CachedTraverse::new().traverse(parts, &nums)
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (parts, nums_str) = line.split_once(" ").unwrap();
            let nums = nums_str
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();
            CachedTraverse::new().traverse(&[parts].repeat(5).join("?"), &nums.repeat(5))
        })
        .sum::<usize>()
        .to_string()
}

struct CachedTraverse<'a> {
    cache: HashMap<(&'a str, &'a [usize]), usize>,
}

impl<'a> CachedTraverse<'a> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn traverse(&mut self, parts: &'a str, nums: &'a [usize]) -> usize {
        if parts.is_empty() {
            if nums.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }

        if nums.is_empty() {
            if parts.contains('#') {
                return 0;
            } else {
                return 1;
            }
        }

        if self.cache.contains_key(&(parts, nums)) {
            return self.cache[&(parts, nums)];
        }

        let mut result = 0;

        if ".?".contains(&parts[..1]) {
            result += self.traverse(&parts[1..], nums);
        }

        if "#?".contains(&parts[..1])
            && nums[0] <= parts.len()
            && !parts[..nums[0]].contains('.')
            && (nums[0] == parts.len() || !parts[nums[0]..nums[0] + 1].contains('#'))
        {
            result += self.traverse(&parts.get(nums[0] + 1..).unwrap_or(""), &nums[1..]);
        }

        self.cache.insert((parts, nums), result);

        result
    }
}
