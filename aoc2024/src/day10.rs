use crate::pos::{Coord, Pos};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[aoc(day10, part1)]
fn part1(map: &[Vec<u8>]) -> u32 {
    let mut ret = 0;
    let startpos: Vec<Pos> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().filter(|(_, &c)| c == 0).map(move |(x, _)| (x, y)))
        .collect();
    for pos in startpos {
        // println!("startpos: {:?} ", pos);
        let mut peaks = HashSet::new();
        let mut current = pos;
        let mut current_height;
        let mut next = VecDeque::new();
        next.push_back(current);
        while !next.is_empty() {
            current = next.pop_front().unwrap();
            current_height = map[current.1][current.0];
            if current_height == 9 {
                peaks.insert(current);
            }
            let nextps: Vec<Pos> = find_next(&current, map)
                .into_iter()
                .filter(|&ps| map[ps.1][ps.0] == current_height + 1)
                .collect();
            for ps in nextps {
                next.push_back(ps);
            }
        }
        ret += peaks.len() as u32;
    }

    ret
}
fn find_next(start: &Pos, input: &[Vec<u8>]) -> Vec<Pos> {
    let posible = vec![start.up(), start.down(input.len()), start.left(), start.right(input[0].len())];
    posible.iter().filter_map(|&p| p).collect()
}

#[aoc(day10, part2)]
fn part2(map: &[Vec<u8>]) -> u32 {
    let mut ret = 0;
    let startpos: Vec<Pos> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().filter(|(_, &c)| c == 0).map(move |(x, _)| (x, y)))
        .collect();
    for pos in startpos {
        let mut current = pos;
        let mut current_height;
        let mut next = VecDeque::new();
        next.push_back(current);
        while !next.is_empty() {
            current = next.pop_front().unwrap();
            current_height = map[current.1][current.0];
            if current_height == 9 {
                ret += 1;
            }
            let nextps: Vec<Pos> = find_next(&current, map)
                .into_iter()
                .filter(|&ps| map[ps.1][ps.0] == current_height + 1)
                .collect();
            for ps in nextps {
                next.push_back(ps);
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        //println!("{:?}", parse(TESTINPUT));
        assert_eq!(part1(&parse(TESTINPUT)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 81);
    }
}
