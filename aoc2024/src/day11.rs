// use std::collections::HashMap;
use rustc_hash::FxHashMap as HashMap;

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
    stone_simulate(input, 25)
}
#[aoc(day11, part2)]
fn part2(input: &[u64]) -> u64 {
    stone_simulate(input, 75)
}
fn stone_simulate(input: &[u64], simlen: u32) -> u64 {
    let mut stones = HashMap::default();
    let mut loopcnt = 0;
    for stone in input {
        stones.insert(*stone, 1);
    }
    for _i in 0..simlen {
        let mut new_stones = HashMap::default();
        for (stone, cnt) in stones.iter() {
            loopcnt += 1;
            match (stone, split(*stone)) {
                (0, _) => {
                    new_stones.entry(1).and_modify(|c| *c += *cnt).or_insert(*cnt);
                }
                (_, Some((n1, n2))) => {
                    new_stones.entry(n1).and_modify(|c| *c += *cnt).or_insert(*cnt);
                    new_stones.entry(n2).and_modify(|c| *c += *cnt).or_insert(*cnt);
                }
                _ => {
                    new_stones.entry(*stone * 2024).and_modify(|c| *c += *cnt).or_insert(*cnt);
                }
            }
        }
        stones = new_stones;
    }
    println!("Total unique numbers after {} blinks: {} fn cnt: {}", simlen, stones.len(), loopcnt);
    stones.iter().map(|(_, v)| v).sum()
}

fn split(a: u64) -> Option<(u64, u64)> {
    let (mut multiplier, mut len) = (1, 0);
    let mut bt = a;
    while bt > 0 {
        bt /= 10;
        if len % 2 == 1 {
            multiplier *= 10;
        }
        len += 1;
    }
    match len % 2 {
        1 => None,
        _ => Some((a / multiplier, a % multiplier)),
    }
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
