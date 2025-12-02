use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(String, String)> {
    let chunks = input.split(',').map(|s| s.trim());
    chunks
        .map(|chunk| {
            let parts: Vec<&str> = chunk.split('-').collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(String, String)]) -> u64 {
    let mut sum = 0;
    for (start, end) in input.iter() {
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        for num in start..=end {
            let num_str = num.to_string();
            if find_repeated(&num_str) {
                sum += num;
            }
        }
    }
    sum
}

#[aoc(day2, part2)]
fn part2(input: &[(String, String)]) -> u64 {
    let mut sum = 0;
    for (start, end) in input.iter() {
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        for num in start..=end {
            let num_str = num.to_string();
            if find_repeated(&num_str) || find_repeated2(&num_str) {
                sum += num;
            }
        }
    }
    sum
}

fn find_repeated(num: &str) -> bool {
    let (p1, p2) = num.split_at(num.len() / 2);
    p1 == p2
}

fn find_repeated2(num: &str) -> bool {
    'outer: for i in 1..=num.len() / 2 {
        let chunk = &num[0..i];
        if chunk != &num[i..2 * i] {
            continue;
        }
        let chunk2 = &num[i..];
        for part in &chunk2.chars().chunks(i) {
            let part_str: String = part.collect();
            if part_str != chunk {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert!(find_repeated2("111"));
        assert_eq!(part2(&parse(TESTINPUT)), 4174379265);
    }
}
