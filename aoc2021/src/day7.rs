use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<i32> {
    input.split(",").map(|x| x.parse::<i32>().unwrap()).collect()
}

#[aoc(day7, part1)]
fn part1(crabs: &[i32]) -> i32 {
    let max = crabs.iter().max().unwrap();
    let mut min_fuel = i32::MAX;
    for pos in 0..=*max {
        let fuel = crabs.iter().fold(0, |acc, x| acc + abs(pos, *x));
        min_fuel = min(fuel, min_fuel);
    }
    min_fuel
}

#[aoc(day7, part2)]
fn part2(crabs: &[i32]) -> i32 {
    let max = crabs.iter().max().unwrap();
    let mut min_fuel = i32::MAX;
    for pos in 0..=*max {
        let fuel = crabs.iter().fold(0, |acc, x| acc + (0..=abs(pos, *x)).sum::<i32>());
        min_fuel = min(fuel, min_fuel);
    }
    min_fuel
}
use std::cmp::{max, min};

fn abs(x: i32, y: i32) -> i32 {
    max(x, y) - min(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "16,1,2,0,4,2,7,1,2,14";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 168);
    }
}
