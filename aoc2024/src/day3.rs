use core::prelude;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;

use nom::bytes::complete::tag;

use nom::combinator::not;
use nom::sequence::preceded;
use nom::{self, IResult};
use nom::{
    character::complete::{anychar, char, digit1},
    multi::{many0, many1, many_till},
    sequence::{delimited, separated_pair},
};

fn parser1(i: &str) -> IResult<&str, (&str, &str)> {
    let (a, (_b, c)) = many_till(
        anychar,
        delimited(
            tag("mul("),
            separated_pair(digit1::<&str, nom::error::Error<&str>>, char(','), digit1),
            char(')'),
        ),
    )(i)?;
    Ok((a, c))
}

fn parser2(i: &str) -> IResult<&str, (String, &str, &str)> {
    let (a, (b, (c, d))) = many_till(
        anychar,
        delimited(
            tag("mul("),
            separated_pair(digit1::<&str, nom::error::Error<&str>>, char(','), digit1),
            char(')'),
        ),
    )(i)?;
    let s: String = b.into_iter().collect();
    Ok((a, (s, c, d)))
}
fn last_do(input: &str) -> IResult<&str, State> {
    let mut state = State::None;
    let dos = |inp| {
        let (ret, (_, d)) = many_till(
            anychar,
            alt((tag("do()"), tag("don't()"))),
        )(inp)?;
        Ok((ret, d))
    };
    
    let (ret, s) = many0(dos)(input)?;
    if let Some(last) = s.last() {
        state = match *last {
            "do()" => State::Do,
            "don't()" => State::Dont,
            _ => State::None,
        };
    }
    Ok((ret, state))
}

#[aoc_generator(day3, part1)]
fn parse1(input: &str) -> Vec<(u32, u32)> {
    let (_a, b) = many1(parser1)(input).unwrap();
    b.iter()
        .map(|(n1, n2)| (n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()))
        .collect()
}
#[derive(Debug)]
enum State {
    Do,
    Dont,
    None,
}
#[aoc_generator(day3, part2)]
fn parse2(input: &str) -> Vec<(u32, u32)> {
    let (_a, b) = many1(parser2)(input).unwrap();
    let mut ret = vec![];
    let mut state = State::None;
    for (prev, s1, s2) in b {
        let (_, lstdo) = last_do(&prev).unwrap();
        match (&state, &lstdo) {
            (State::None | State::Do, State::None) => {
                ret.push((s1.parse().unwrap(), s2.parse().unwrap()))
            }
            (_, State::Do) => {
                ret.push((s1.parse().unwrap(), s2.parse().unwrap()));
                state = State::Do;
            }
            (_, State::Dont) => state = State::Dont,
            _ => (),
        };
    }
    ret
}

#[aoc(day3, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().fold(0, |acc, (n1, n2)| acc + (n1 * n2))
}

#[aoc(day3, part2)]
fn part2(input: &[(u32, u32)]) -> u32 {
    input.iter().fold(0, |acc, (n1, n2)| acc + (n1 * n2))
}

#[cfg(test)]
mod tests {

    use super::*;
    const TESTINPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TESTINPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(TESTINPUT1)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(TESTINPUT2)), 48);
    }
}
