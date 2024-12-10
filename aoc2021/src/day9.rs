use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as u8))
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[u8]) -> u32 {
    todo!()
}

#[aoc(day9, part2)]
fn part2(input: &[u8]) -> u32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;
const TESTINPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 83);
    }
}