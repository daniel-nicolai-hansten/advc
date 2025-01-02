use crate::str_to_static;
use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use itertools::Itertools;

use std::collections::HashMap;
use std::sync::OnceLock;
static LOOKUP: OnceLock<HashMap<[char; 2], char>> = OnceLock::new();
#[aoc_generator(day14)]
fn parse(input: &str) -> (&'static str, HashMap<[char; 2], char>) {
    let input = str_to_static(input);
    let mut lookup = HashMap::new();
    let mut template = None;
    for line in input.lines() {
        match template {
            None => template = Some(line),
            Some(_) => {
                if let Some((ptrn, c)) = line.split_once(" -> ") {
                    let c = c.chars().nth(0).unwrap();
                    let c1 = ptrn.chars().nth(0).unwrap();
                    let c2 = ptrn.chars().nth(1).unwrap();
                    lookup.insert([c1, c2], c);
                }
            }
        }
    }
    (template.unwrap(), lookup)
}

#[aoc(day14, part1)]
fn part1(input: &(&str, HashMap<[char; 2], char>)) -> usize {
    let mut tots = [0; 27];
    let (template, lookup) = input;
    let _ = LOOKUP.set(lookup.clone());
    let cnt = 10;
    template.chars().for_each(|c| tots[alf_idx(c)] += 1);
    for (c1, c2) in template.chars().tuple_windows() {
        let ar1 = polymer((c1, c2), cnt);
        tots = sum_arr(ar1, tots);
    }
    let max = tots.iter().max().unwrap();
    let min = tots.iter().filter(|n| n != &&0).min().unwrap();
    max - min
}

#[aoc(day14, part2)]
fn part2(input: &(&str, HashMap<[char; 2], char>)) -> usize {
    let mut tots = [0; 27];
    let (template, lookup) = input;
    let _ = LOOKUP.set(lookup.clone());
    let cnt = 40;
    template.chars().for_each(|c| tots[alf_idx(c)] += 1);
    for (c1, c2) in template.chars().tuple_windows() {
        let ar1 = polymer((c1, c2), cnt);
        tots = sum_arr(ar1, tots);
    }
    let max = tots.iter().max().unwrap();
    let min = tots.iter().filter(|n| n != &&0).min().unwrap();
    max - min
}
fn alf_idx(c: char) -> usize {
    (c as u8 - 65) as usize
}
fn sum_arr(a1: [usize; 27], a2: [usize; 27]) -> [usize; 27] {
    let mut a1 = a1;
    for (i, n) in a2.iter().enumerate() {
        a1[i] += n;
    }
    a1
}
#[cached]
fn polymer(pair: (char, char), cnt: usize) -> [usize; 27] {
    let mut ret = [0; 27];
    match pair {
        (c1, c3) if cnt != 0 => {
            if let Some(&c2) = LOOKUP.get().unwrap().get(&[c1, c3]) {
                let ar1 = polymer((c1, c2), cnt - 1);
                let ar2 = polymer((c2, c3), cnt - 1);
                ret = sum_arr(ar1, ar2);
                ret[alf_idx(c2)] += 1;
            }
        }
        _ => (),
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 1588);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 2188189693529);
    }
}
