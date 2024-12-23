use std::collections::VecDeque;

use cached::proc_macro::cached;
use itertools::Itertools;

use crate::pos::{Coord, Pos};
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

#[aoc(day21, part1)]
fn part1(input: &[String]) -> usize {
    let mut ret = 0;
    for code in input {
        let len = get_sequence(code.clone(), 2, Pads::NumPad);
        let code: usize = code.strip_suffix("A").unwrap().parse().unwrap();
        ret += len * code;
    }
    ret
}

#[aoc(day21, part2)]
fn part2(input: &[String]) -> usize {
    let mut ret = 0;
    for code in input {
        let len = get_sequence(code.clone(), 25, Pads::NumPad);
        let code: usize = code.strip_suffix("A").unwrap().parse().unwrap();
        ret += len * code;
    }
    ret
}
#[rustfmt::skip]
const NUMPAD: [[char; 3]; 4] =
    [
        ['7', '8', '9'], 
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A']
    ];
#[rustfmt::skip]
const DIRPAD: [[char; 3]; 2] =
    [
        [' ', '^', 'A'],
        ['<', 'v', '>']
    ];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pads {
    NumPad,
    DirPad,
}
impl Pads {
    fn get(&self) -> &'static [[char; 3]] {
        match self {
            Self::NumPad => &NUMPAD,
            Self::DirPad => &DIRPAD,
        }
    }
}
fn find_path(target: char, pad: Pads, start: char) -> Vec<String> {
    let pad = pad.get();
    let start_pos = pad
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&x| x == start).map(|x| Pos::new(x, y)))
        .unwrap();
    let len = pad.len();
    let mut ret = Vec::new();
    let path: Vec<char> = Vec::new();
    let mut bestlen = usize::MAX;
    let mut queue = VecDeque::new();
    let visited = Vec::new();
    queue.push_back((start_pos, path, visited));
    while let Some((pos, path, mut visited)) = queue.pop_front() {
        if pad[pos.y()][pos.x()] == target {
            if path.len() > bestlen {
                break;
            }
            bestlen = path.len().min(bestlen);
            ret.push(path);
            continue;
        }
        visited.push(pos);
        for (newpos, dir) in pos.neighbors_dir(len, pad[0].len()) {
            if pad[newpos.y()][newpos.x()] != ' ' && !visited.contains(&newpos) {
                let mut path = path.clone();

                path.push(dir.into());
                queue.push_back((newpos, path, visited.clone()));
            }
        }
    }
    ret.iter().map(|x| x.iter().collect()).collect()
}

#[cached]
fn get_sequence(sequence: String, depth: usize, pad: Pads) -> usize {
    let target = "A".to_string() + &sequence;
    let mut ret = 0;
    for (a, b) in target.chars().tuple_windows() {
        let paths = find_path(b, pad, a);
        ret += match depth {
            0 => paths[0].len() + 1,
            _ => paths.iter().cloned().map(|path| {
                    get_sequence(path + "A", depth - 1, Pads::DirPad)
                })
                .min()
                .unwrap(),
        }
    }
    ret
}

#[cfg(test)]
mod tests {

    use super::*;
    const TESTINPUT: &str = "029A
980A
179A
456A
379A";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 154115708116294);
    }
}
