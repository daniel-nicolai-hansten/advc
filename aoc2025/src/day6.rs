use std::vec;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day6, part1)]
fn parse1(input: &str) -> (Vec<Vec<u64>>, Vec<Ops>) {
    let mut ops = Vec::new();
    let mut numbers = Vec::new();
    for line in input.lines() {
        match &line[0..1] {
            "*" | "+" => {
                for sym in line.split_whitespace() {
                    match sym {
                        "*" => ops.push(Ops::Mul),
                        "+" => ops.push(Ops::Add),
                        _ => (),
                    }
                }
            }
            _ => {
                let nums = line.split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<u64>>();
                numbers.push(nums);
            }
        }
    }
    (numbers, ops)
}
#[aoc_generator(day6, part2)]
fn parse2(input: &str) -> (Vec<Vec<(usize, u64)>>, Vec<(usize, Ops)>) {
    let mut nums = Vec::new();
    let mut ops = Vec::new();
    for line in input.lines() {
        let mut group = 0;
        let mut last_c = None;
        for (idx, c) in line.chars().enumerate() {
            if nums.len() <= idx {
                nums.push(Vec::new());
            }
            match (c, last_c) {
                (c, _) if c.is_numeric() => {
                    let val = c.to_digit(10).unwrap() as u64;
                    nums[idx].push((group, val));
                }
                (' ', Some(' ')) | (' ', None) => continue,
                (' ', Some(_)) => {
                    group += 1;
                }
                ('*', _) => {
                    ops.push((group, Ops::Mul));
                }
                ('+', _) => {
                    ops.push((group, Ops::Add));
                }
                c => panic!("Unexpected character in input {:?}", c),
            }
            last_c = Some(c);
        }
    }
    (nums, ops)
}
#[derive(Debug)]
enum Ops {
    Add,
    Mul,
}
#[aoc(day6, part1)]
fn part1(input: &(Vec<Vec<u64>>, Vec<Ops>)) -> u64 {
    let (numbers, ops) = input;
    let mut results = Vec::new();
    for nums in numbers {
        for (idx, num) in nums.iter().enumerate() {
            let num = match (results.get(idx), ops.get(idx)) {
                (Some(&res), Some(Ops::Add)) => res + num,
                (Some(&res), Some(Ops::Mul)) => res * num,
                _ => {
                    results.push(0);
                    *num
                }
            };
            *results.get_mut(idx).unwrap() = num;
        }
    }
    results.iter().sum()
}

#[aoc(day6, part2)]
fn part2(input: &(Vec<Vec<(usize, u64)>>, Vec<(usize, Ops)>)) -> u64 {
    let (numbers, ops) = input;
    let mut results = vec![];
    for nums in numbers {
        if nums.is_empty() {
            continue;
        }
        let group = nums[0].0;
        let numsum = nums.iter().rev().enumerate().fold(0, |acc, (idx, (_grp, n))| n * 10_u64.pow(idx as u32) + acc);
        while results.len() <= group {
            results.push(0);
        }
        match ops.iter().find(|(g, _)| *g == group) {
            Some((_, Ops::Add)) => results[group] += numsum,
            Some((_, Ops::Mul)) => results[group] = numsum * results[group].max(1),
            None => panic!("No operation found for group {}", group),
        }
    }
    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(TESTINPUT)), 4277556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(TESTINPUT)), 3263827);
    }
}
