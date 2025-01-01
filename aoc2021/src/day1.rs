use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input.windows(2).fold(0, |acc, x| if x[0] < x[1] { acc + 1 } else { acc })
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    input
        .windows(3)
        .map(|x| x.iter().sum::<u32>())
        .tuple_windows()
        .fold(0, |acc, (x1, x2)| if x1 < x2 { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 5);
    }
}
