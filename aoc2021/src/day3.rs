use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    let mut counters = vec![];
    for line in input {
        while line.len() > counters.len() {
            counters.push(0);
        }
        for (i, bit) in line.iter().enumerate() {
            counters[i] += match bit {
                1 => 1,
                0 => -1,
                _ => 0,
            };
        }
    }
    let mut gamma = 0;
    for (i, n) in counters.iter().rev().enumerate() {
        if *n >= 0 {
            gamma |= 1 << i;
        }
    }
    let mut epsilon = 0;
    for (i, n) in counters.iter().rev().enumerate() {
        if *n < 0 {
            epsilon |= 1 << i;
        }
    }
    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    let len = input[0].len();
    let nums = input
        .iter()
        .map(|line| line.iter().enumerate().fold(0, |acc, (i, x)| acc | x << len - 1 - i))
        .collect::<Vec<u32>>();
    let calculate_bit_criteria = |list: &[u32], bitnum: usize| {
        let mut counters = 0;
        for i in list {
            match i & (1 << len - 1 - bitnum) == 0 {
                true => counters -= 1,
                false => counters += 1,
            }
        }
        counters >= 0
    };
    let (mut oxygen_generator_rating, mut co2_scrubber_rating) = (0, 0);
    let mut currentnums = nums.to_vec();
    for i in 0..len {
        if !calculate_bit_criteria(&currentnums, i) {
            currentnums.retain(|x| *x & (1 << len - 1 - i) == 0);
        } else {
            currentnums.retain(|x| *x & (1 << len - 1 - i) != 0);
        }
        if currentnums.len() == 1 {
            oxygen_generator_rating = currentnums[0];
            break;
        }
    }
    let mut currentnums = nums.to_vec();
    for i in 0..len {
        if !calculate_bit_criteria(&currentnums, i) {
            currentnums.retain(|x| *x & (1 << len - 1 - i) != 0);
        } else {
            currentnums.retain(|x| *x & (1 << len - 1 - i) == 0);
        }
        if currentnums.len() == 1 {
            co2_scrubber_rating = currentnums[0];
            break;
        }
    }

    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
0101";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 198);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 230);
    }
}
