use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;

use nom::bytes::complete::tag;

use nom::character::complete;
use nom::combinator::value;
use nom::{self, IResult};
use nom::{
    character::complete::anychar,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
};
#[derive(Debug, Clone, Copy)]
enum Instr {
    Mul(u32, u32),
    Do,
    Dont,
}
fn mul(input: &str) -> IResult<&str, Instr> {
    let (input, _) = tag("mul")(input)?;
    let (input, (n1, n2)) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instr::Mul(n1, n2)))
}

fn instruction(input: &str) -> IResult<&str, Instr> {
    let (rest, (_, input)) = many_till(
        anychar,
        alt((
            value(Instr::Dont, tag("don't()")),
            value(Instr::Do, tag("do()")),
            mul,
        )),
    )(input)?;
    Ok((rest, input))
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Instr> {
    let (_, b) = many1(instruction)(input).unwrap();
    b
}

#[aoc(day3, part1)]
fn part1(input: &[Instr]) -> u32 {
    input.iter().fold(0, |acc, ins| {
        if let Instr::Mul(n1, n2) = ins {
            acc + (n1 * n2)
        } else {
            acc
        }
    })
}

#[aoc(day3, part2)]
fn part2(input: &[Instr]) -> u32 {
    let mut state = true;
    input
        .iter()
        .map(|inst| match inst {
            Instr::Mul(n1, n2) if state => n1 * n2,
            Instr::Do => {
                state = true;
                0
            }
            Instr::Dont => {
                state = false;
                0
            }
            _ => 0,
        })
        .sum()
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
        assert_eq!(part1(&parse(TESTINPUT1)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT2)), 48);
    }
}
