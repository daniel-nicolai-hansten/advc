use aoc_runner_derive::{aoc, aoc_generator};
use nom::{branch::alt, bytes::complete::tag, character::complete, error::Error, sequence::preceded};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let fwd = |i| preceded(tag("forward "), complete::i32::<&str, Error<&str>>)(i).map(|(i2, n)| (i2, Command::Fwd(n)));
    let dwn = |i| preceded(tag("down "), complete::i32)(i).map(|(i2, n)| (i2, Command::Dwn(n)));
    let up = |i| preceded(tag("up "), complete::i32)(i).map(|(i2, n)| (i2, Command::Up(n)));
    for line in input.lines() {
        let (_i, o) = alt((fwd, dwn, up))(line).unwrap();
        commands.push(o);
    }
    commands
}
enum Command {
    Fwd(i32),
    Dwn(i32),
    Up(i32),
}

#[aoc(day2, part1)]
fn part1(input: &[Command]) -> i32 {
    let (mut x, mut y) = (0, 0);
    for command in input {
        match command {
            Command::Fwd(n) => x += n,
            Command::Dwn(n) => y += n,
            Command::Up(n) => y -= n,
        }
    }
    x * y
}

#[aoc(day2, part2)]
fn part2(input: &[Command]) -> i32 {
    let (mut x, mut y, mut aim) = (0, 0, 0);
    for command in input {
        match command {
            Command::Fwd(n) => {
                x += n;
                y += aim * n;
            }
            Command::Dwn(n) => aim += n,
            Command::Up(n) => aim -= n,
        }
    }
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 150);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 900);
    }
}
