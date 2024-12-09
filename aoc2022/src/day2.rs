use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{self, alpha1, multispace1, newline, one_of},
    multi::separated_list1,
    sequence::separated_pair,
};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(char, char)> {
    let (_input, lst) = separated_list1(
        newline::<&str, nom::error::Error<&str>>,
        separated_pair(one_of("ABC"), multispace1, one_of("XYZ")),
    )(input)
    .unwrap();
    lst
}

#[aoc(day2, part1)]
fn part1(input: &[(char, char)]) -> String {
    todo!()
}

#[aoc(day2, part2)]
fn part2(input: &[(char, char)]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
