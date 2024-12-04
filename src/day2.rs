fn main() {
    let data: Vec<Vec<i64>> = include_str!("../data/day_2024_2.data")
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|item| item.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("Part 1 : {}", part1(&data));
    println!("Part 2 : {}", part2(&data));
}

fn is_valid_report(input: &[i64]) -> bool {
    input.windows(2).all(|win| {
        let diff = (win[0] - win[1]).abs();
        diff > 0 && diff <= 3
    }) && {
        let mut cloned = input.to_vec();
        cloned.sort_unstable();
        let reversed: Vec<_> = cloned.iter().copied().rev().collect();
        cloned == input || reversed == input
    }
}

fn is_valid_report_part2(input: &[i64]) -> bool {
    is_valid_report(input)
        || (0..input.len()).any(|index| {
            let mut clone = input.to_vec();
            clone.remove(index);
            is_valid_report(&clone)
        })
}

pub fn part1(reports: &[Vec<i64>]) -> usize {
    reports
        .iter()
        .filter(|report| is_valid_report(report))
        .count()
}

pub fn part2(reports: &[Vec<i64>]) -> usize {
    reports
        .iter()
        .filter(|report| is_valid_report_part2(report))
        .count()
}
