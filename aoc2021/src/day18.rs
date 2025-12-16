use aoc_runner_derive::{aoc, aoc_generator};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    multi::many1,
    number::{
        complete::{le_u32, u32 as nom_u32},
        le_u32,
    },
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};
#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<SnailfishNumber> {
    many1(parse_snailfish).parse(input).unwrap().1
}
enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Regular(u32),
}

fn parse_snailfish(input: &str) -> IResult<&str, SnailfishNumber> {
    let (rest, n) = alt(
        le_u32.map(|n| SnailfishNumber::Regular(n)),
        delimited(
            tag("["),
            separated_pair(parse_snailfish, tag(","), parse_snailfish).map(|(l, r)| SnailfishNumber::Pair(Box::new(l), Box::new(r))),
            tag("]"),
        ),
    )
    .parse(input)?;
    Ok((rest, n))
}

#[aoc(day18, part1)]
fn part1(_input: &[SnailfishNumber]) -> String {
    todo!()
}

#[aoc(day18, part2)]
fn part2(_input: &[SnailfishNumber]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let _input = parse("[1,2]");

        // assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        // assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
