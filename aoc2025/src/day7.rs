use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use std::collections::HashSet;
#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<Vec<char>>) -> u64 {
    let start = (input[0].iter().position(|&c| c == 'S').unwrap(), 0);
    let mut beams = HashSet::new();
    let mut splits = 0;
    let mut wq = vec![start];
    while let Some((x, y)) = wq.pop() {
        if y >= input.len() || x >= input[0].len() {
            continue;
        }
        match input[y][x] {
            '^' => {
                let n = [(x + 1, y + 1), (x.wrapping_sub(1), y + 1)];
                splits += 1;
                for n1 in n {
                    if beams.insert(n1) {
                        wq.push(n1);
                    }
                }
            }
            '.' | 'S' => {
                let n = (x, y + 1);
                if beams.insert(n) {
                    wq.push(n);
                }
            }
            _ => {}
        }
    }
    splits
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Vec<char>>) -> u64 {
    let start = (input[0].iter().position(|&c| c == 'S').unwrap(), 0);
    let splits = find_path(start, input);
    splits
}

#[cached(key = "(usize, usize)", convert = r#"{pos }"#)]
fn find_path(pos: (usize, usize), map: &Vec<Vec<char>>) -> u64 {
    let (x, y) = pos;
    if y >= map.len() || x >= map[0].len() {
        return 1;
    }
    match map[y][x] {
        '^' => {
            let left = find_path((x.wrapping_sub(1), y + 1), map);
            let right = find_path((x + 1, y + 1), map);
            left + right
        }
        '.' | 'S' => find_path((x, y + 1), map),
        _ => 0,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 40);
    }
}
