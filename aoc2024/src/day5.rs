use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{self, line_ending};
use nom::character::is_digit;
use nom::combinator::{opt, value};

use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{self, IResult};
use nom::{
    character::complete::anychar,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
};

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let (input, rules) = many1(terminated(parse_rule, line_ending))(input).unwrap();
    let (_, (_, pagelist)) = many_till(anychar, many1(parse_pglist))(input).unwrap();
    (rules, pagelist)
}
#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32,
}
impl Rule {
    fn validate(&self, pages: &[u32]) -> bool {
        let firstidx = pages.iter().position(|&x| x == self.first);
        let secondidx = pages.iter().position(|&x| x == self.second);
        match (firstidx, secondidx) {
            (Some(idx1), Some(idx2)) if idx1 > idx2 => false,
            _ => true,
        }
    }
}
impl From<(u32, u32)> for Rule {
    fn from((first, second): (u32, u32)) -> Self {
        Self { first, second }
    }
}
fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (first, second)) = separated_pair(complete::u32, tag("|"), complete::u32)(input)?;
    Ok((input, Rule::from((first, second))))
}
fn parse_pglist(input: &str) -> IResult<&str, Vec<u32>> {
    let (inp, res) = terminated(separated_list1(tag(","), complete::u32), opt(line_ending))(input)?;
    Ok((inp, res))
}
#[aoc(day5, part1)]
fn part1(input: &(Vec<Rule>, Vec<Vec<u32>>)) -> u32 {
    let (rules, pagelist) = input;
    let mut cnt = 0;
    println!("{}  {}", rules.len(), pagelist.len());
    'outer: for pages in pagelist {
        for rule in rules {
            if !rule.validate(pages) {
                continue 'outer;
            }
        }
        cnt += pages[pages.len() / 2];
    }
    cnt
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<Rule>, Vec<Vec<u32>>)) -> u32 {
    let (rules, pagelist) = input;
    let mut sort_pages = vec![];
    let mut cnt = 0;
    for pages in pagelist {
        for rule in rules {
            if !rule.validate(pages) {
                let pages_tosort = pages.clone();
                sort_pages.push(pages_tosort);
                break;
            }
        }
    }
    for pages in sort_pages {
        sorted.sort_by
        cnt += sorted[sorted.len() / 2];
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT1)), 143);
    }

    #[test]
    fn part2_example() {
        //assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
