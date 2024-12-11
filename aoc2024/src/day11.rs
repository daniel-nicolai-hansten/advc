use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::character::complete;
use nom::IResult;
use nom::{character::complete::space1, multi::separated_list1};
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    parse2(input).unwrap().1
}
fn parse2(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> u64 {
    let mut stones = input.to_vec();
    for _i in 0..25 {
        let mut new_stones = Vec::with_capacity(stones.len() * 2);
        for stone in stones.iter() {
            match (stone, split(*stone)) {
                (0, _) => new_stones.push(1),

                (_, Some((n1, n2))) => {
                    new_stones.push(n1);
                    new_stones.push(n2);
                }
                _ => new_stones.push(*stone * 2024),
            }
        }
        stones = new_stones;
    }
    stones.len() as u64
}
fn split(a: u64) -> Option<(u64, u64)> {
    let mut multiplier = 1;
    let mut bt = a;
    let mut ln = 0;
    while bt > 0 {
        bt /= 10;
        if ln % 2 == 1 {
            multiplier *= 10;
        }
        ln += 1;
    }
    match ln % 2 {
        1 => None,
        _ => Some((a / multiplier, a % multiplier)),
    }
}
#[aoc(day11, part2)]
fn part2(input: &[u64]) -> u64 {
    let mut stones = HashMap::new();
    for stone in input {
        stones.insert(*stone, 1);
    }
    for _i in 0..75 {
        let mut new_stones = HashMap::new();
        for (stone, cnt) in stones.iter() {
            match (stone, split(*stone)) {
                (0, _) => {
                    if let Some(c) = new_stones.insert(1, *cnt) {
                        new_stones.insert(1, c + cnt);
                    }
                }

                (_, Some((n1, n2))) => {
                    if let Some(c) = new_stones.insert(n1, *cnt) {
                        new_stones.insert(n1, c + cnt);
                    }
                    if let Some(c) = new_stones.insert(n2, *cnt) {
                        new_stones.insert(n2, c + cnt);
                    }
                }
                _ => {
                    if let Some(c) = new_stones.insert(*stone * 2024, *cnt) {
                        new_stones.insert(*stone * 2024, c + cnt);
                    }
                }
            }
        }
        stones = new_stones;
    }
    stones.iter().map(|(_, v)| v).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "125 17";
    #[test]
    fn part1_example() {
        assert_eq!(split(234), None);
        assert_eq!(split(1234), Some((12, 34)));
        assert_eq!(part1(&parse(TESTINPUT)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 65601038650482);
    }
}
