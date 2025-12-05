use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u64>]) -> u64 {
    input.iter().map(|banks| findnum(banks, 2)).sum()
}

fn findnum(banks: &[u64], digits: usize) -> u64 {
    let (mut num, mut pos) = (0, 0);
    for digit in (0..digits).rev() {
        let bankslice = &banks[pos..banks.len() - digit];
        let volt = bankslice.iter().max().unwrap();
        pos = bankslice.iter().position(|&v| v == *volt).unwrap() + 1 + pos;
        num += *volt * 10u64.pow(digit as u32);
    }
    num
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u64>]) -> u64 {
    input.iter().fold(0, |acc, banks| findnum(banks, 12) + acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 3121910778619);
    }
}
