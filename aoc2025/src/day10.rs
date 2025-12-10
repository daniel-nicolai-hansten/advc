use aoc_runner_derive::{aoc, aoc_generator};
use nom::Parser;
use nom::character::complete::{char, one_of};
use nom::multi::{many1, separated_list1};
use nom::{IResult, bytes::complete::tag, sequence::delimited};
use std::vec;
#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)> {
    let (mut lights, mut switches, mutjolts) = (vec![], vec![], vec![]);
    let mut ret = vec![];
    for line in input.lines() {
        let (rest, v) = lineparse(line).unwrap();
        ret.push(v);
    }
    (vec![], vec![], vec![])
}
fn lineparse(input: &str) -> IResult<&str, (Vec<bool>, Vec<Vec<u64>>, Vec<u64>)> {
    let (input, bools) = delimited(tag("["), many1(one_of(".#").map(|c| c == '#')), tag("]")).parse(input)?;
    let (input, switches) = many1(delimited(tag("("), separated_list1(char(','), nom::character::complete::u64), tag(")"))).parse(input)?;
    let (input, jolts) = delimited(tag("{"), separated_list1(char(','), nom::character::complete::u64), tag("}")).parse(input)?;
    Ok((input, (bools, switches, jolts)))
}

fn parse_bool_array(input: &str) -> nom::IResult<&str, Vec<bool>> {
    let (input, chars) = delimited(tag("["), many1(one_of(".#")), tag("]")).parse(input)?;

    let bools = chars.into_iter().map(|c| c == '#').collect();

    Ok((input, bools))
}

#[aoc(day10, part1)]
fn part1(input: &(Vec<bool>, Vec<u64>, Vec<u64>)) -> u64 {
    todo!()
}

#[aoc(day10, part2)]
fn part2(input: &(Vec<bool>, Vec<u64>, Vec<u64>)) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 50);
    }
}
