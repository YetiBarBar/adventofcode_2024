use regex::Regex;

fn main() {
    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let input = include_str!("../data/day_2024_3.data");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(data: &str) -> i64 {
    let re = Regex::new(r"mul\((?<val1>\d{1,3}),(?<val2>\d{1,3})\)").unwrap();

    re.captures_iter(data)
        .map(|capture| (capture.name("val1").unwrap(), capture.name("val2").unwrap()))
        .map(|(a, b)| {
            (
                a.as_str().parse::<i64>().unwrap(),
                b.as_str().parse::<i64>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum()
}

//
fn part2(data: &str) -> i64 {
    let re =
        Regex::new(r"mul\((?<val1>\d{1,3}),(?<val2>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))")
            .unwrap();

    re.captures_iter(data)
        .fold((true, 0), |(mut status, mut res), capture| {
            if capture.name("do").is_some() {
                status = true;
            } else if capture.name("dont").is_some() {
                status = false;
            }
            if status {
                if let (Some(a), Some(b)) = (capture.name("val1"), capture.name("val2")) {
                    res += a.as_str().parse::<i64>().unwrap() * b.as_str().parse::<i64>().unwrap();
                }
            }
            (status, res)
        })
        .1
}
