use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let (_, r) = parsein(input).unwrap();
    r
}

fn parsein(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    let parse_line = |input| {
        separated_pair(
            map_res(digit1, u64::from_str),
            tag(": "),
            separated_list1(space1, map_res(digit1, u64::from_str)),
        )(input)
    };
    separated_list1(tag("\n"), parse_line)(input)
}

#[aoc(day7, part1)]
fn part1(input: &[(u64, Vec<u64>)]) -> u64 {
    input.into_iter().fold(0, |acc, (num, nums)| {
        let nums: Vec<u64> = nums.iter().rev().map(|n| *n).collect();
        match recp1(&nums, *num) {
            true => acc + *num,
            false => acc,
        }
    })
}

#[aoc(day7, part2)]
fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    input.into_iter().fold(0, |acc, (num, nums)| {
        let nums: Vec<u64> = nums.iter().rev().map(|n| *n).collect();
        match recp2(&nums, *num) {
            true => acc + *num,
            false => acc,
        }
    })
}

pub fn recp1(nums: &[u64], num: u64) -> bool {
    match nums {
        [] => false,
        [single] => *single == num,
        [last, rest @ ..] => {
            if num % last == 0 && recp1(rest, num / last) {
                return true;
            }
            if num > *last && recp1(rest, num - last) {
                return true;
            }
            false
        }
    }
}

pub fn recp2(nums: &[u64], num: u64) -> bool {
    match nums {
        [] => false,
        [single] => *single == num,
        [last, rest @ ..] => {
            let (_, multiplier) = concat(num, *last);
            if num % multiplier == *last && recp2(rest, num / multiplier) {
                return true;
            }
            if num % last == 0 && recp2(rest, num / last) {
                return true;
            }
            if num > *last && recp2(rest, num - last) {
                return true;
            }
            false
        }
    }
}

fn concat(a: u64, b: u64) -> (u64, u64) {
    let mut multiplier = 1;
    let mut bt = b;
    while bt > 0 {
        bt /= 10;
        multiplier *= 10;
    }
    (a * multiplier + b, multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(concat(300, 20), (30020, 100));
        assert_eq!(part2(&parse(TESTINPUT)), 11387);
    }
}
