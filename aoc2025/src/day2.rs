use aoc_runner_derive::{aoc, aoc_generator};
use tinyvec::ArrayVec;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    let chunks = input.split(',').map(|s| s.trim());
    chunks
        .map(|chunk| {
            let parts = chunk.split_once('-').unwrap();
            (parts.0.parse::<u64>().unwrap(), parts.1.parse::<u64>().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    let mut num_str = ArrayVec::<[u8; 20]>::new();
    for (start, end) in input.iter() {
        for num in *start..=*end {
            numtovec(num, &mut num_str);
            let (p1, p2) = num_str.split_at(num_str.len() / 2);
            if p1 == p2 {
                sum += num;
            }
        }
    }
    sum
}

#[aoc(day2, part2)]
fn part2(input: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    let mut num_arr = ArrayVec::<[u8; 20]>::new();
    for (start, end) in input.iter() {
        for num in *start..=*end {
            numtovec(num, &mut num_arr);
            if find_repeated(&num_arr) {
                sum += num;
            }
        }
    }
    sum
}

fn numtovec(mut num: u64, vec: &mut ArrayVec<[u8; 20]>) {
    vec.clear();
    while num > 0 {
        vec.push((num % 10) as u8);
        num /= 10;
    }
}

fn find_repeated(num: &[u8]) -> bool {
    for i in 2..=num.len() / 2 {
        if num.len() % i != 0 {
            continue;
        }
        let mut chunks = num.chunks(i);
        if let Some(first) = chunks.next() {
            if chunks.all(|chunk| chunk == first) {
                return true;
            }
        }
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
        assert_eq!(part2(&parse(TESTINPUT)), 4174379265);
    }
}
