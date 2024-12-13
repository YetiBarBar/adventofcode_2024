use std::collections::HashMap;

#[derive(PartialEq, Clone)]
struct PageOrdered<'a> {
    page_number: usize,
    ruleset: &'a HashMap<usize, Vec<usize>>,
}

impl PartialOrd for PageOrdered<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ruleset != other.ruleset {
            return None;
        }
        if self.page_number == other.page_number {
            return Some(std::cmp::Ordering::Equal);
        }
        if let Some(value) = self.ruleset.get(&self.page_number) {
            if value.contains(&other.page_number) {
                return Some(std::cmp::Ordering::Less);
            }
        }
        if let Some(value) = self.ruleset.get(&other.page_number) {
            if value.contains(&self.page_number) {
                return Some(std::cmp::Ordering::Greater);
            }
        }

        None
    }
}

fn main() {
    let data = include_str!("../data/day_2024_5.data");

    let (rules, pages) = data.split_once("\n\n").unwrap();

    let rulesset: HashMap<usize, Vec<usize>> =
        rules.lines().fold(HashMap::new(), |mut map, line| {
            let (key, val) = line.split_once('|').unwrap();
            let key = key.parse::<usize>().unwrap();
            let val = val.parse::<usize>().unwrap();
            map.entry(key).or_default().push(val);
            map
        });

    let pages: Vec<Vec<PageOrdered>> = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|val| {
                    val.parse::<usize>()
                        .map(|val| PageOrdered {
                            page_number: val,
                            ruleset: &rulesset,
                        })
                        .unwrap()
                })
                .collect()
        })
        .collect();

    println!("Part 1: {}", part1(&pages));
    println!("Part 2: {}", part2(&pages));
}

fn part1(book: &[Vec<PageOrdered>]) -> usize {
    book.iter()
        .filter_map(|pages| {
            let mut cloned = pages.clone();
            cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if &cloned == pages {
                Some(cloned)
            } else {
                None
            }
        })
        .map(|line: Vec<PageOrdered>| line.get(line.len() / 2).cloned().unwrap())
        .map(|page_ordered| page_ordered.page_number)
        .sum()
}

fn part2(book: &[Vec<PageOrdered>]) -> usize {
    book.iter()
        .filter_map(|pages| {
            let mut cloned = pages.clone();
            cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if &cloned != pages {
                Some(cloned)
            } else {
                None
            }
        })
        .map(|line: Vec<PageOrdered>| line.get(line.len() / 2).cloned().unwrap())
        .map(|page_ordered| page_ordered.page_number)
        .sum()
}
