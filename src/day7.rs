use adventofcode_tooling::AocError;
use std::str::FromStr;

struct Equation {
    goal: u128,
    values: Vec<u128>,
}

impl FromStr for Equation {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (goal, values) = s.split_once(": ").unwrap();
        let goal = goal.parse().unwrap();
        let values = values
            .split_ascii_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        Ok(Self { goal, values })
    }
}

impl Equation {
    fn is_valid_part1(&self) -> bool {
        let reachable = self.values.iter().fold(vec![0_u128], |acc, item| {
            let mut new_acc = vec![];
            for value in acc {
                new_acc.push(value * item);
                new_acc.push(value + item);
            }
            new_acc
        });
        reachable.contains(&self.goal)
    }
}
/* fn eval_slice(in_data: &[u128]) -> Vec<u128> {
if in_data.len() == 1 {
    in_data.to_vec()
} else if in_data.len() == 2 {
    vec![
        in_data[0] * in_data[1],
        in_data[0] + in_data[1],
        format!("{}{}", in_data[0], in_data[1]).parse().unwrap(),
    ]
} else {
    let evaluated = eval_slice(&in_data[1..]);
    let mut new_vec= vec![in_data[0] * in_data[1],
    in_data[0] + in_data[1],
    format!("{}{}", in_data[0], in_data[1]).parse().unwrap(),
    ];
    for value in evaluated {
        new_vec.push(in_data[0])
    }
} */

fn part1(data: &[Equation]) -> u128 {
    data.iter()
        .filter(|equation| equation.is_valid_part1())
        .map(|equation| equation.goal)
        .sum()
}

fn main() {
    let data: Vec<Equation> = include_str!("../data/day_2024_7.data")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {}", part1(&data));
}
