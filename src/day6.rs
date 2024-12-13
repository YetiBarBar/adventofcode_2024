use std::collections::HashSet;
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Heading {
    fn next_heading(&self) -> Self {
        match &self {
            Heading::Up => Heading::Right,
            Heading::Down => Heading::Left,
            Heading::Left => Heading::Up,
            Heading::Right => Heading::Down,
        }
    }
}

pub fn main() {
    let mut data: Vec<Vec<char>> = include_str!("../data/day_2024_6.data")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let (guard_x, guard_y) = {
        let (mut guard_x, mut guard_y) = (0, 0);
        for y in 0..data.len() {
            for x in 0..data[0].len() {
                if data[y][x] == '^' {
                    (guard_x, guard_y) = (x, y);
                }
            }
        }
        (guard_x, guard_y)
    };
    data[guard_y][guard_x] = '.';
    let heading = Heading::Up;

    let mut visited_part1 = part1(&data, guard_x, guard_y, heading);

    println!("Part 1: {}", visited_part1.len());
    println!(
        "Part 2: {}",
        part2(&data, guard_x, guard_y, heading, &mut visited_part1)
    )
}

fn part1(
    data: &[Vec<char>],
    guard_x: usize,
    guard_y: usize,
    heading: Heading,
) -> HashSet<(usize, usize)> {
    let mut guard_x = guard_x;
    let mut guard_y = guard_y;
    let mut heading = heading;
    let mut visited = HashSet::new();
    visited.insert((guard_x, guard_y));
    while let Some((next_x, next_y, next_heading)) = next_position(data, guard_x, guard_y, heading)
    {
        visited.insert((next_x, next_y));
        guard_x = next_x;
        guard_y = next_y;
        heading = next_heading;
    }

    visited
}

fn part2(
    data: &[Vec<char>],
    guard_x: usize,
    guard_y: usize,
    heading: Heading,
    visited_part1: &mut HashSet<(usize, usize)>,
) -> usize {
    visited_part1.remove(&(guard_x, guard_y));
    let mut count = 0;
    for (point_x, point_y) in visited_part1.iter() {
        let mut new_x = guard_x;
        let mut new_y = guard_y;
        let mut new_heading = heading;
        let mut visited = HashSet::new();
        visited.insert((guard_x, guard_y, heading));
        while let Some((next_x, next_y, next_heading)) =
            next_position_part2(data, new_x, new_y, new_heading, *point_x, *point_y)
        {
            if visited.contains(&(next_x, next_y, next_heading)) {
                count += 1;
                break;
            }
            visited.insert((next_x, next_y, next_heading));
            new_x = next_x;
            new_y = next_y;
            new_heading = next_heading;
        }
    }
    count
}

fn next_position(
    data: &[Vec<char>],
    pos_x: usize,
    pos_y: usize,
    heading: Heading,
) -> Option<(usize, usize, Heading)> {
    let (next_x, next_y) = match heading {
        Heading::Up => (pos_x, pos_y.wrapping_sub(1)),
        Heading::Down => (pos_x, pos_y + 1),
        Heading::Left => (pos_x.wrapping_sub(1), pos_y),
        Heading::Right => (pos_x + 1, pos_y),
    };

    if next_x >= data[0].len() || next_y >= data.len() {
        return None;
    }
    let chr = data[next_y][next_x];
    if chr == '.' {
        Some((next_x, next_y, heading))
    } else {
        Some((pos_x, pos_y, heading.next_heading()))
    }
}

fn next_position_part2(
    data: &[Vec<char>],
    pos_x: usize,
    pos_y: usize,
    heading: Heading,
    faked_x: usize,
    faked_y: usize,
) -> Option<(usize, usize, Heading)> {
    let (next_x, next_y) = match heading {
        Heading::Up => (pos_x, pos_y.wrapping_sub(1)),
        Heading::Down => (pos_x, pos_y + 1),
        Heading::Left => (pos_x.wrapping_sub(1), pos_y),
        Heading::Right => (pos_x + 1, pos_y),
    };

    if next_x >= data[0].len() || next_y >= data.len() {
        return None;
    }
    if next_x == faked_x && next_y == faked_y {
        return Some((pos_x, pos_y, heading.next_heading()));
    }
    let chr = data[next_y][next_x];
    if chr == '.' {
        Some((next_x, next_y, heading))
    } else {
        Some((pos_x, pos_y, heading.next_heading()))
    }
}
