use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
#[aoc_generator(day17)]
fn parse(input: &str) -> ((i64, i64), (i64, i64)) {
    parse_nom(input).unwrap().1
}
fn parse_nom(input: &str) -> IResult<&str, ((i64, i64), (i64, i64))> {
    let range = |input| {
        let (rest, _axis) = one_of("xy").parse(input)?;
        let (rest, start) = preceded(tag("="), nom::character::complete::i64).parse(rest)?;
        let (rest, end) = preceded(tag(".."), nom::character::complete::i64).parse(rest)?;
        Ok((rest, (start, end)))
    };
    let (rest, parsed) = preceded(tag("target area: "), separated_pair(range, tag(", "), range)).parse(input)?;
    Ok((rest, parsed))
}

#[aoc(day17, part1)]
fn part1(input: &((i64, i64), (i64, i64))) -> i64 {
    let mut highest = i64::MIN;

    for x in -1000..1000 {
        for y in -200..200 {
            let mut position = (0, 0);
            let mut projectile = (x, y);
            let mut highest_tmp = i64::MIN;
            loop {
                position = (position.0 + projectile.0, position.1 + projectile.1);
                highest_tmp = highest_tmp.max(position.1);
                projectile = projectile.step();
                if position.0 >= input.0 .0 && position.0 <= input.0 .1 && position.1 >= input.1 .0 && position.1 <= input.1 .1 {
                    highest = highest.max(highest_tmp);
                    break;
                }
                if position.0 > input.0 .1 || position.1 < input.1 .0 {
                    break;
                }
            }
        }
    }
    highest
}
type Projectile = (i64, i64);
trait ProjectileExt {
    fn step(&self) -> Self;
}
impl ProjectileExt for Projectile {
    fn step(&self) -> Self {
        let (mut vx, mut vy) = *self;
        vx = match vx {
            0 => 0,
            n if n > 0 => n - 1,
            n if n < 0 => n + 1,
            _ => unreachable!(),
        };
        vy -= 1;
        (vx, vy)
    }
}

#[aoc(day17, part2)]
fn part2(input: &((i64, i64), (i64, i64))) -> usize {
    let mut hits = 0;
    for x in -1000..1000 {
        for y in -200..200 {
            let mut position = (0, 0);
            let mut projectile = (x, y);
            let mut highest = i64::MIN;
            loop {
                position = (position.0 + projectile.0, position.1 + projectile.1);
                highest = highest.max(position.1);
                projectile = projectile.step();
                if position.0 >= input.0 .0 && position.0 <= input.0 .1 && position.1 >= input.1 .0 && position.1 <= input.1 .1 {
                    hits += 1;
                    break;
                }
                if position.0 > input.0 .1 || position.1 < input.1 .0 {
                    break;
                }
            }
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "target area: x=20..30, y=-10..-5";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 45);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(TESTINPUT)), 50);
    // }
}
