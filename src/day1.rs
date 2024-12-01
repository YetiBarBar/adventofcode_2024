use std::collections::HashMap;

fn main() {
    let (mut left, mut right) = include_str!("../data/day_2024_1.data").lines().fold(
        (vec![], vec![]),
        |(mut l, mut r), line| {
            let items = line
                .split_ascii_whitespace()
                .map(|item| item.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            l.push(items[0]);
            r.push(items[1]);
            (l, r)
        },
    );

    left.sort_unstable();
    right.sort_unstable();

    println!("Part 1 : {}", part1(&left, &right));
    println!("Part 2 : {}", part2(&left, &right));
}

pub fn part1(left: &[i64], right: &[i64]) -> i64 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn part2(left: &[i64], right: &[i64]) -> i64 {
    let right_map = right.iter().fold(HashMap::new(), |mut hmap, item| {
        *hmap.entry(item).or_insert(0_i64) += 1;
        hmap
    });

    left.iter()
        .map(|val| right_map.get(val).unwrap_or(&0) * val)
        .sum()
}
