// use nohash::IntMap as HashMap;
// use nohash::IntSet as HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{character::complete::newline, multi::separated_list1};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;
#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<u32> {
    let (_i, o) = separated_list1(newline::<&str, nom::error::Error<&str>>, nom::character::complete::u32)(input).unwrap();
    o
}

#[aoc(day22, part1)]
fn part1(input: &[u32]) -> u64 {
    let mut ret = 0;
    for i in input {
        let mut random = std::iter::successors(Some(*i), |&n| Some(rng(n)));
        ret += random.nth(2000).unwrap() as u64;
    }
    ret
}

#[aoc(day22, part2)]
fn part2(input: &[u32]) -> u32 {
    let mut diff_candidates = HashMap::default();
    let mut max = 0;
    let mut diff_add = |diffwindow: u32, num: u32| {
        let price = num % 10;

        diff_candidates
            .entry(diffwindow)
            .and_modify(|x| {
                *x += price as u32;
                max = max.max(*x);
            })
            .or_insert(price);
    };

    let find_diff = |a: u32, b: u32| {
        let a = (a % 10) as i8;
        let b = (b % 10) as i8;
        (a - b).to_ne_bytes()[0]
    };

    for i in input {
        let random = std::iter::successors(Some(*i), |&n| Some(rng(n)));
        let mut diffs_seen = HashSet::default();
        for (i, (n0, n1, n2, n3, n4)) in random.tuple_windows().enumerate() {
            let diffwindow = u32::from_ne_bytes([find_diff(n0, n1), find_diff(n1, n2), find_diff(n2, n3), find_diff(n3, n4)]);
            if diffs_seen.insert(diffwindow) {
                diff_add(diffwindow, n4);
            }
            if i > 2000 {
                break;
            }
        }
    }

    max
}
fn rng(num: u32) -> u32 {
    let mut num = num as u64;
    let mix = |n1, n2| n1 ^ n2;
    let prune = |n| n % 16777216;
    num = prune(mix(num * 64, num));
    num = prune(mix(num / 32, num));
    num = prune(mix(num * 2048, num));
    num as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "1
10
100
2024";

    const TESTINPUT2: &str = "1
2
3
2024";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT2)), 23);
    }
}
