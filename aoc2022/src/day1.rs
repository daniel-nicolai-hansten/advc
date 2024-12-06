use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{self, newline},
    multi::{many1, separated_list1},
};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    let (_input, lst) = separated_list1(many1(newline::<&str, nom::error::Error<&str>>), separated_list1(newline, complete::u32))(input).unwrap();
    lst
}

#[aoc(day1, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    let mut lst = input.iter().map(|x| x.iter().sum::<u32>()).collect::<Vec<u32>>();
    lst.sort();
    *lst.last().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    let mut lst = input.iter().map(|x| x.iter().sum::<u32>()).collect::<Vec<u32>>();
    lst.sort();
    lst[(lst.len() - 3)..lst.len()].iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 24000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 45000);
    }
}
