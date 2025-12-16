use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<SnailfishNumber> {
    separated_list1(newline, parse_snailfish).parse(input).unwrap().1
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Regular(u32),
}
impl SnailfishNumber {
    fn magnitude(&self) -> u32 {
        match self {
            SnailfishNumber::Regular(n) => *n,
            SnailfishNumber::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
    fn add(self, other: SnailfishNumber) -> SnailfishNumber {
        SnailfishNumber::Pair(Box::new(self), Box::new(other))
    }
    fn reduce(&mut self) {
        loop {
            if self.explode(0).0 {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }
    fn split(&mut self) -> bool {
        match self {
            SnailfishNumber::Regular(n) if *n >= 10 => {
                let left = *n / 2;
                let right = (*n + 1) / 2;
                *self = SnailfishNumber::Pair(Box::new(SnailfishNumber::Regular(left)), Box::new(SnailfishNumber::Regular(right)));
                true
            }
            SnailfishNumber::Pair(l, r) => {
                if l.split() {
                    true
                } else {
                    r.split()
                }
            }
            _ => false,
        }
    }
    fn explode(&mut self, depth: usize) -> (bool, Option<u32>, Option<u32>) {
        match self {
            SnailfishNumber::Pair(l, r) if depth >= 4 => {
                if let (SnailfishNumber::Regular(left), SnailfishNumber::Regular(right)) = (&**l, &**r) {
                    let left_value = *left;
                    let right_value = *right;
                    *self = SnailfishNumber::Regular(0);
                    return (true, Some(left_value), Some(right_value));
                }
                (false, None, None)
            }
            SnailfishNumber::Pair(l, r) => {
                let (exploded, left_add, right_add) = l.explode(depth + 1);
                if exploded {
                    if let Some(rv) = right_add {
                        r.add_to_leftmost(rv);
                    }
                    return (true, left_add, None);
                }
                let (exploded, left_add, right_add) = r.explode(depth + 1);
                if exploded {
                    if let Some(lv) = left_add {
                        l.add_to_rightmost(lv);
                    }
                    return (true, None, right_add);
                }
                (false, None, None)
            }
            _ => (false, None, None),
        }
    }
    fn add_to_leftmost(&mut self, value: u32) {
        match self {
            SnailfishNumber::Regular(n) => *n += value,
            SnailfishNumber::Pair(l, _) => l.add_to_leftmost(value),
        }
    }
    fn add_to_rightmost(&mut self, value: u32) {
        match self {
            SnailfishNumber::Regular(n) => *n += value,
            SnailfishNumber::Pair(_, r) => r.add_to_rightmost(value),
        }
    }
}

fn parse_snailfish(input: &str) -> IResult<&str, SnailfishNumber> {
    let (rest, n) = alt((
        digit1.map(|s: &str| SnailfishNumber::Regular(s.parse().unwrap())),
        delimited(
            tag("["),
            separated_pair(parse_snailfish, tag(","), parse_snailfish).map(|(l, r)| SnailfishNumber::Pair(Box::new(l), Box::new(r))),
            tag("]"),
        ),
    ))
    .parse(input)?;
    Ok((rest, n))
}

#[aoc(day18, part1)]
fn part1(input: &[SnailfishNumber]) -> u32 {
    let mut snail_iter = input.into_iter();
    let mut first = snail_iter.next().unwrap().clone();
    for sn in snail_iter {
        first = first.add(sn.clone());
        first.reduce();
    }
    first.magnitude()
}

#[aoc(day18, part2)]
fn part2(input: &[SnailfishNumber]) -> u32 {
    let mut max_magnitude = 0;
    for number in input.iter().permutations(2) {
        let mut sum = number[0].clone().add(number[1].clone());
        sum.reduce();
        max_magnitude = max_magnitude.max(sum.magnitude());
    }
    max_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 4140);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 3993);
    }
}
