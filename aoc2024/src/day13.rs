use std::cmp::min;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    sequence::{preceded, separated_pair},
    IResult,
};

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<ClawMachine> {
    input.split("\n\n").map(|x| parse_ruleset(x).unwrap().1).collect()
}
fn parse_ruleset(i: &str) -> IResult<&str, ClawMachine> {
    let (i, a) = preceded(
        tag("Button A: "),
        separated_pair(preceded(tag("X+"), complete::i128), tag(", "), preceded(tag("Y+"), complete::i128)),
    )(i)?;
    let (i, b) = preceded(
        preceded(newline, tag("Button B: ")),
        separated_pair(preceded(tag("X+"), complete::i128), tag(", "), preceded(tag("Y+"), complete::i128)),
    )(i)?;
    let (i, p) = preceded(
        preceded(newline, tag("Prize: ")),
        separated_pair(preceded(tag("X="), complete::i128), tag(", "), preceded(tag("Y="), complete::i128)),
    )(i)?;
    Ok((
        i,
        ClawMachine {
            button_a: a,
            button_b: b,
            prize: p,
        },
    ))
}
#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize: (i128, i128),
}

#[aoc(day13, part1)]
fn part1(input: &[ClawMachine]) -> i128 {
    let mut res = 0;
    'outer: for claw in input {
        let maxpresses = 100;
        for i in 0..maxpresses {
            let bpresses = maxpresses - i;
            let rem = claw.prize.0 as i32 - (claw.button_b.0 * bpresses) as i32;
            if rem < 0 {
                continue;
            }
            if rem % claw.button_a.0 as i32 == 0 {
                let apresses = rem as i128 / claw.button_a.0;
                let pos_x = (claw.button_a.0 * apresses) + (claw.button_b.0 * bpresses);
                let pos_y = (claw.button_a.1 * apresses) + (claw.button_b.1 * bpresses);
                if (pos_x, pos_y) == claw.prize {
                    res += bpresses + (apresses * 3);
                    continue 'outer;
                }
            }
        }
    }
    res
}

#[aoc(day13, part2)]
fn part2(input: &[ClawMachine]) -> i128 {
    let mut res = 0;
    for ClawMachine { button_a, button_b, prize } in input {
        let mut prize = prize.clone();
        prize.0 += 10000000000000;
        prize.1 += 10000000000000;
        let num_a = prize.1 * button_b.0 - prize.0 * button_b.1;
        let denom_a = button_a.1 * button_b.0 - button_a.0 * button_b.1;
        let num_b = prize.1 * button_a.0 - prize.0 * button_a.1;
        let denom_b = 0 - denom_a;
        if num_a % denom_a != 0 || num_b % denom_b != 0 {
            continue;
        }

        res += num_a / denom_a * 3 + num_b / denom_b;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 480);
    }
}
