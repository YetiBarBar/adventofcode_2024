use adventofcode_tooling::Matrix2D;
fn main() {
    /*     let _input_data: Vec<&str> = r"MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX"
            .lines()
            .collect(); */

    let input_data: Vec<&str> = include_str!("../data/day_2024_4.data").lines().collect();
    let line_len = input_data[0].len();
    let col_count = input_data.len();
    let chars = input_data.iter().flat_map(|line| line.chars()).collect();

    let data: Matrix2D<char> = Matrix2D {
        width: line_len,
        height: col_count,
        values: chars,
    };

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(data: &Matrix2D<char>) -> u64 {
    let mut count = 0;
    for x in 0..data.width {
        for y in 0..data.height {
            count += found_down(data, x, y) as u64;
            count += found_up(data, x, y) as u64;
            count += found_left(data, x, y) as u64;
            count += found_right(data, x, y) as u64;
            count += found_right_down(data, x, y) as u64;
            count += found_left_down(data, x, y) as u64;
            count += found_left_up(data, x, y) as u64;
            count += found_right_up(data, x, y) as u64;
        }
    }
    count
}

fn part2(data: &Matrix2D<char>) -> u64 {
    let mut count = 0;
    for x in 0..data.width {
        for y in 0..data.height {
            if found_cross_mas_diag(data, x, y) {
                count += 1;
            }
        }
    }
    count
}

fn found_right(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    let row = data.row(x);
    row.iter().skip(y).take(4).collect::<String>() == "XMAS"
}

fn found_down(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    let col = data.col(y);
    col.iter().skip(x).take(4).collect::<String>() == "XMAS"
}
fn found_left(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    let row = data.row(x);
    row.iter().skip(y).take(4).collect::<String>() == "SAMX"
}

fn found_up(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    let col = data.col(y);
    col.iter().skip(x).take(4).collect::<String>() == "SAMX"
}

fn found_left_up(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    y >= 3
        && x >= 3
        && data.get(x, y) == Some('X')
        && data.get(x - 1, y - 1) == Some('M')
        && data.get(x - 2, y - 2) == Some('A')
        && data.get(x - 3, y - 3) == Some('S')
}

fn found_right_up(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    y >= 3
        && data.get(x, y) == Some('X')
        && data.get(x + 1, y - 1) == Some('M')
        && data.get(x + 2, y - 2) == Some('A')
        && data.get(x + 3, y - 3) == Some('S')
}
fn found_right_down(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    data.get(x, y) == Some('X')
        && data.get(x + 1, y + 1) == Some('M')
        && data.get(x + 2, y + 2) == Some('A')
        && data.get(x + 3, y + 3) == Some('S')
}

fn found_left_down(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    x >= 3
        && data.get(x, y) == Some('X')
        && data.get(x - 1, y + 1) == Some('M')
        && data.get(x - 2, y + 2) == Some('A')
        && data.get(x - 3, y + 3) == Some('S')
}

fn found_cross_mas_diag(data: &Matrix2D<char>, x: usize, y: usize) -> bool {
    if data.get(x, y) == Some('A') {
        let mut count_m = 0;
        let mut count_s = 0;
        if x > 0 {
            if y > 0 {
                let val = data.get(x - 1, y - 1);
                if matches!(val, Some('M')) {
                    count_m += 1;
                }
                if matches!(val, Some('S')) {
                    count_s += 1;
                }
            }
            let val = data.get(x - 1, y + 1);
            if matches!(val, Some('M')) {
                count_m += 1;
            }
            if matches!(val, Some('S')) {
                count_s += 1;
            }
        }

        if y > 0 {
            let val = data.get(x + 1, y - 1);
            if matches!(val, Some('M')) {
                count_m += 1;
            }
            if matches!(val, Some('S')) {
                count_s += 1;
            }
        }
        let val = data.get(x + 1, y + 1);
        if matches!(val, Some('M')) {
            count_m += 1;
        }
        if matches!(val, Some('S')) {
            count_s += 1;
        }
        count_m == 2 && count_s == 2 && data.get(x - 1, y - 1) != data.get(x + 1, y + 1)
    } else {
        false
    }
}
