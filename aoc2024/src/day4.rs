use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect::<Vec<char>>()).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<char>]) -> u32 {
    let mut cnt = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if vertical(input, Pos { x, y }) {
                cnt += 1;
            }
            if horizontal(input, Pos { x, y }) {
                cnt += 1;
            }
            if diagonal(input, Pos { x, y }) {
                cnt += 1;
            }
            if diagonal2(input, Pos { x, y }) {
                cnt += 1;
            }
        }
    }
    cnt
}
struct Pos {
    x: usize,
    y: usize,
}
fn vertical(map: &[Vec<char>], startpos: Pos) -> bool {
    let mut collected = vec![];
    for i in 0..4 {
        let x = startpos.x + i;
        if x >= map[startpos.y].len() {
            return false;
        }
        collected.push(map[startpos.y][x]);
    }
    match collected.iter().collect::<String>().as_str() {
        "XMAS" => true,
        "SAMX" => true,
        _ => false,
    }
}
fn horizontal(map: &[Vec<char>], startpos: Pos) -> bool {
    let mut collected = vec![];
    for i in 0..4 {
        let y = startpos.y + i;
        if y >= map.len() {
            return false;
        }
        collected.push(map[y][startpos.x]);
    }
    match collected.iter().collect::<String>().as_str() {
        "XMAS" => true,
        "SAMX" => true,
        _ => false,
    }
}
fn diagonal(map: &[Vec<char>], startpos: Pos) -> bool {
    let mut collected = vec![];
    for i in 0..4 {
        let x = startpos.x + i;
        let y = startpos.y + i;
        if x >= map[startpos.y].len() || y >= map.len() {
            return false;
        }
        collected.push(map[y][x]);
    }
    match collected.iter().collect::<String>().as_str() {
        "XMAS" => true,
        "SAMX" => true,
        _ => false,
    }
}
fn diagonal2(map: &[Vec<char>], startpos: Pos) -> bool {
    let mut collected = vec![];
    for i in 0..4 {
        let x = startpos.x.wrapping_sub(i);
        let y = startpos.y + i;
        if x >= map[startpos.y].len() || y >= map.len() {
            return false;
        }
        collected.push(map[y][x]);
    }
    match collected.iter().collect::<String>().as_str() {
        "XMAS" => true,
        "SAMX" => true,
        _ => false,
    }
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<char>]) -> u32 {
    let mut cnt = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if find_xmas(input, Pos { x, y }) {
                cnt += 1;
            }
        }
    }
    cnt
}
fn find_xmas(map: &[Vec<char>], startpos: Pos) -> bool {
    let mut collected1 = vec![];
    let mut collected2 = vec![];
    for i in 0..3 {
        let x1 = startpos.x + i;
        let y1 = startpos.y + i;
        let x2 = (startpos.x + 2).wrapping_sub(i);
        let y2 = startpos.y + i;
        if x1 >= map[startpos.y].len() || y1 >= map.len() || x2 >= map[startpos.y].len() || y2 >= map.len() {
            return false;
        }

        collected1.push(map[y1][x1]);
        collected2.push(map[y2][x2]);
    }
    match (collected1.iter().collect::<String>().as_str(), collected2.iter().collect::<String>().as_str()) {
        ("MAS" | "SAM", "MAS" | "SAM") => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
    #[test]
    fn part1_example() {
        // assert_eq!(vertical(&parse(TESTINPUT1), Pos { x: 0, y: 4 }), true);
        // assert_eq!(horizontal(&parse(TESTINPUT1), Pos { x: 9, y: 6 }), true);
        assert_eq!(part1(&parse(TESTINPUT1)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT1)), 9);
    }
}
