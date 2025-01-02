use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of},
    error::Error,
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair},
    Parser,
};

use crate::pos::{Coord, Pos};

#[aoc_generator(day13)]
fn parse(input: &str) -> (Vec<Pos>, Vec<FoldOp>) {
    let (i, pos) = separated_list0(
        line_ending::<&str, Error<&str>>,
        separated_pair(complete::u32.map(|n| n as usize), tag(","), complete::u32.map(|n| n as usize)),
    )(input)
    .unwrap();
    let (i, _o) = many0(line_ending::<&str, Error<&str>>)(i).unwrap();
    let (_i, ops) = separated_list0(
        line_ending,
        preceded(
            tag::<&str, &str, Error<&str>>("fold along "),
            separated_pair(one_of("xy"), tag("="), complete::u32).map(|(c, n)| match c {
                'x' => FoldOp::X(n as usize),
                'y' => FoldOp::Y(n as usize),
                _ => unreachable!(),
            }),
        ),
    )(i)
    .unwrap();
    (pos, ops)
}
enum FoldOp {
    X(usize),
    Y(usize),
}
trait Fold: Coord {
    fn fold(&self, op: &FoldOp) -> Pos {
        let ts = |n, fld: usize| match n > fld {
            true => fld - fld.abs_diff(n),
            false => n,
        };
        match op {
            FoldOp::X(fld) => (ts(self.x(), *fld), self.y()),
            FoldOp::Y(fld) => (self.x(), ts(self.y(), *fld)),
        }
    }
}
impl Fold for Pos {}

#[aoc(day13, part1)]
fn part1(input: &(Vec<Pos>, Vec<FoldOp>)) -> usize {
    let mut dots = input.0.clone();
    let ops = input.1.as_slice();
    dots.iter_mut().for_each(|p| *p = p.fold(&ops[0]));
    dots.sort_unstable();
    dots.dedup();
    dots.len()
}

#[aoc(day13, part2)]
fn part2(input: &(Vec<Pos>, Vec<FoldOp>)) -> String {
    let mut dots = input.0.clone();
    let ops = input.1.as_slice();
    for op in ops {
        dots.iter_mut().for_each(|p| *p = p.fold(op));
    }
    let max_x = dots.iter().max_by_key(|p| p.x()).unwrap().x();
    let max_y = dots.iter().max_by_key(|p| p.y()).unwrap().y();
    let mut map = String::new();
    map.push_str("\n");
    for y in 0..=max_y {
        let mut line = String::new();
        for x in 0..=max_x {
            let c = match dots.iter().find(|&p| p == &(x, y)) {
                Some(_) => '#',
                None => ' ',
            };
            line.push(c);
        }
        map.push_str(&line);
        map.push_str("\n");
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 17);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), "\n#####\n#   #\n#   #\n#   #\n#####\n");
    }
}
