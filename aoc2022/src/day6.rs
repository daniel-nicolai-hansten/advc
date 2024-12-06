use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| 1 << (c.to_digit(36).unwrap() - 10)).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[u32]) -> usize {
    find_marker(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &[u32]) -> usize {
    find_marker(input, 14)
}
fn find_marker(input: &[u32], len: usize) -> usize {
    let mut map = input[0..len - 1].iter().fold(0, |acc, x| acc ^ x);
    for (idx, slce) in input.windows(len).enumerate() {
        map ^= slce.last().unwrap();
        if map.count_ones() == len as u32 {
            return idx + len;
        }
        map ^= slce.first().unwrap();
    }
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TESTINPUT2: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT2)), 26);
    }
}
