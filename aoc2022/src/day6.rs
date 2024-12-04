use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| 1 << (c.to_digit(36).unwrap() - 10))
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[u32]) -> String {
    input.windows(4).

    format!("marker found at ")
}

#[aoc(day6, part2)]
fn part2(input: &[u32]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    #[test]
    fn part1_example() {
        let test = parse("aaabbbcccddd");
        println!("{:?}", test);
        assert_eq!(part1(&parse(TESTINPUT)), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
