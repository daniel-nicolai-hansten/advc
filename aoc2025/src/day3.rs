use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u8>]) -> u64 {
    let mut sum = 0;
    for banks in input {
        let volt = banks[..banks.len() - 1].iter().max().unwrap();
        let pos = banks.iter().position(|&v| v == *volt).unwrap();
        let volt2 = banks[pos + 1..].iter().max_by_key(|&v| v).unwrap();
        let volt = volt * 10 + volt2;
        sum += volt as u64;
    }
    sum
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u8>]) -> u64 {
    let mut sum = 0;

    for banks in input {
        let mut numarr = vec![];
        let mut pos = 0;
        for digit in (0..12).rev() {
            let bankslice = &banks[pos..banks.len() - digit];
            let volt = bankslice.iter().max().unwrap();
            pos = bankslice.iter().position(|&v| v == *volt).unwrap() + 1 + pos;
            numarr.push(*volt);
        }
        sum += numarr.iter().rev().enumerate().map(|(i, &d)| (d as u64) * 10u64.pow(i as u32)).sum::<u64>();
    }
    sum
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
