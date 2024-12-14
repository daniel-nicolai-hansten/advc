use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, character::complete::{self, space1}, sequence::{preceded, separated_pair}, IResult};

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}
#[aoc_generator(day14)]
fn parse(input: &str) -> String {
    todo!()
}
fn parse_line(line: &str) -> IResult<&str, Robot> {
    let (i,o ) = separated_pair(preceded(tag("p="), separated_pair(complete::i32, tag(","), complete::i32)), space1, preceded(tag("v="), separated_pair(complete::i32, tag(","), complete::i32)))(line)?;
}

#[aoc(day14, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;
const TESTINPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}