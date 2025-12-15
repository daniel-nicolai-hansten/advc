use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::collections::HashMap;

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    prse(input)
        .unwrap()
        .1
        .iter()
        .map(|x| {
            (
                x.0.iter().map(|x| x.chars().sorted().collect::<String>()).collect(),
                x.1.iter().map(|x| x.chars().sorted().collect::<String>()).collect(),
            )
        })
        .collect()
}
fn prse(input: &str) -> IResult<&str, Vec<(Vec<&str>, Vec<&str>)>> {
    separated_list1(
        line_ending,
        separated_pair(separated_list1(space1, alpha1), tag(" | "), separated_list1(space1, alpha1)),
    )
    .parse(input)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum SegNum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Unknown,
}
impl SegNum {
    fn len(&self) -> usize {
        match self {
            SegNum::One => 2,
            SegNum::Two => 5,
            SegNum::Three => 5,
            SegNum::Four => 4,
            SegNum::Five => 5,
            SegNum::Six => 6,
            SegNum::Seven => 3,
            SegNum::Eight => 7,
            SegNum::Nine => 6,
            SegNum::Zero => 6,
            SegNum::Unknown => 0,
        }
    }
    fn val(&self) -> u32 {
        match self {
            SegNum::One => 1,
            SegNum::Two => 2,
            SegNum::Three => 3,
            SegNum::Four => 4,
            SegNum::Five => 5,
            SegNum::Six => 6,
            SegNum::Seven => 7,
            SegNum::Eight => 8,
            SegNum::Nine => 9,
            SegNum::Zero => 0,
            SegNum::Unknown => 0,
        }
    }
    fn all() -> Vec<SegNum> {
        vec![
            SegNum::One,
            SegNum::Two,
            SegNum::Three,
            SegNum::Four,
            SegNum::Five,
            SegNum::Six,
            SegNum::Seven,
            SegNum::Eight,
            SegNum::Nine,
            SegNum::Zero,
        ]
    }
}

#[aoc(day8, part1)]
fn part1(inputs: &[(Vec<String>, Vec<String>)]) -> u32 {
    let mut cnt = 0;
    for input in inputs {
        let mut map: Vec<(String, Vec<SegNum>)> = input.0.iter().map(|x| (x.clone(), SegNum::all())).collect();
        let mut found = HashMap::new();
        for (seg, segs) in &mut map {
            segs.retain(|&x| seg.len() == x.len());
            if segs.len() == 1 {
                found.insert(seg.clone(), segs[0]);
            }
        }
        for nbr in input.1.iter() {
            if found.contains_key(nbr) {
                cnt += 1;
            }
        }
    }
    cnt
}
fn find_shared(n1: &str, n2: &str, segnum: SegNum) -> Vec<SegNum> {
    let shared = n1.chars().filter(|&x| n2.contains(x)).count();
    match (segnum, shared) {
        (SegNum::Zero, 2) => vec![SegNum::One],
        (SegNum::Zero, 3) => vec![SegNum::Four, SegNum::Seven],
        (SegNum::Zero, 4) => vec![SegNum::Two, SegNum::Three, SegNum::Five],
        (SegNum::Zero, 5) => vec![SegNum::Six, SegNum::Nine],
        (SegNum::Zero, 6) => vec![SegNum::Eight, SegNum::Zero],
        (SegNum::One, 1) => vec![SegNum::Two, SegNum::Five, SegNum::Six],
        (SegNum::One, 2) => vec![
            SegNum::Zero,
            SegNum::One,
            SegNum::Three,
            SegNum::Four,
            SegNum::Seven,
            SegNum::Eight,
            SegNum::Nine,
        ],
        (SegNum::Two, 1) => vec![SegNum::One],
        (SegNum::Two, 2) => vec![SegNum::Four, SegNum::Seven, SegNum::Nine],
        (SegNum::Two, 3) => vec![SegNum::Five],
        (SegNum::Two, 4) => vec![SegNum::Zero, SegNum::Three, SegNum::Six],
        (SegNum::Two, 5) => vec![SegNum::Two],
        (SegNum::Three, 2) => vec![SegNum::One],
        (SegNum::Three, 3) => vec![SegNum::Four, SegNum::Seven],
        (SegNum::Three, 4) => vec![SegNum::Two, SegNum::Zero, SegNum::Five, SegNum::Six],
        (SegNum::Three, 5) => vec![SegNum::Eight, SegNum::Nine, SegNum::Three],
        (SegNum::Four, 2) => vec![SegNum::One, SegNum::Two, SegNum::Seven],
        (SegNum::Four, 3) => vec![SegNum::Zero, SegNum::Three, SegNum::Five, SegNum::Six],
        (SegNum::Four, 4) => vec![SegNum::Four, SegNum::Eight, SegNum::Nine],
        (SegNum::Five, 1) => vec![SegNum::One],
        (SegNum::Five, 2) => vec![SegNum::Seven],
        (SegNum::Five, 3) => vec![SegNum::Four, SegNum::Two],
        (SegNum::Five, 4) => vec![SegNum::Zero, SegNum::Three],
        (SegNum::Five, 5) => vec![SegNum::Eight, SegNum::Nine, SegNum::Five, SegNum::Six],
        (SegNum::Six, 1) => vec![SegNum::One],
        (SegNum::Six, 2) => vec![SegNum::Seven],
        (SegNum::Six, 3) => vec![SegNum::Four],
        (SegNum::Six, 4) => vec![SegNum::Two, SegNum::Three],
        (SegNum::Six, 5) => vec![SegNum::Zero, SegNum::Five, SegNum::Nine],
        (SegNum::Six, 6) => vec![SegNum::Six, SegNum::Eight],
        (SegNum::Seven, 2) => vec![SegNum::One, SegNum::Four, SegNum::Two, SegNum::Five, SegNum::Six],
        (SegNum::Seven, 3) => vec![SegNum::Zero, SegNum::Three, SegNum::Seven, SegNum::Eight, SegNum::Nine],
        (SegNum::Nine, 2) => vec![SegNum::One],
        (SegNum::Nine, 3) => vec![SegNum::Seven],
        (SegNum::Nine, 4) => vec![SegNum::Four, SegNum::Two],
        (SegNum::Nine, 5) => vec![SegNum::Zero, SegNum::Three, SegNum::Five, SegNum::Six],
        (SegNum::Nine, 6) => vec![SegNum::Eight, SegNum::Nine],
        _ => SegNum::all().into_iter().filter(|b| b.len() == n1.len()).collect(),
    }
    .into_iter()
    .collect()
}

#[aoc(day8, part2)]
fn part2(inputs: &[(Vec<String>, Vec<String>)]) -> u32 {
    let mut cnt = 0;
    for input in inputs {
        let mut map: Vec<(String, Vec<SegNum>)> = input.0.iter().map(|x| (x.clone(), SegNum::all())).collect();
        let mut found: HashMap<SegNum, String> = HashMap::new();
        found.insert(SegNum::Unknown, "".to_string());
        while found.len() <= 10 {
            for segnum in &found {
                for (seg, segs) in &mut map {
                    segs.retain(|&x| find_shared(seg, segnum.1, *segnum.0).contains(&x));
                }
            }
            for (seg, segs) in &map {
                if segs.len() == 1 {
                    found.insert(segs[0], seg.to_string());
                }
            }
        }
        let hm_found = found.into_iter().map(|(k, v)| (v, k)).collect::<HashMap<_, _>>();
        let mut numstack = Vec::new();
        for nbr in input.1.iter() {
            if let Some(seg) = hm_found.get(nbr) {
                numstack.push(seg.val());
            }
        }
        cnt += numstack.iter().fold(0, |acc, x| acc * 10 + x);
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT2: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT2)), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT2)), 61229);
    }
}
