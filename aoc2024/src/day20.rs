use std::collections::VecDeque;

use crate::pos::{Coord, Pos};
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet as HashSet;

#[cfg(not(test))]
const MINSAVE: usize = 100;
#[cfg(test)]
const MINSAVE: usize = 50;

#[aoc_generator(day20)]
fn parse(input: &str) -> (Pos, Pos, Vec<Vec<bool>>) {
    let map = input.lines().map(|s| s.chars().map(|c| c != '#').collect()).collect();
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, s)| s.chars().enumerate().find_map(|(x, c)| (c == 'S').then(|| x)).and_then(|x| Some((x, y))))
        .unwrap();
    let end = input
        .lines()
        .enumerate()
        .find_map(|(y, s)| s.chars().enumerate().find_map(|(x, c)| (c == 'E').then(|| x)).and_then(|x| Some((x, y))))
        .unwrap();
    (start, end, map)
}

#[aoc(day20, part1)]
fn part1(input: &(Pos, Pos, Vec<Vec<bool>>)) -> usize {
    let (start, end, map) = input;
    find_shortcuts(map, start, end, 2)
}

#[aoc(day20, part2)]
fn part2(input: &(Pos, Pos, Vec<Vec<bool>>)) -> usize {
    let (start, end, map) = input;
    find_shortcuts(map, start, end, 20)
}
fn find_shortcuts(map: &[Vec<bool>], start: &Pos, end: &Pos, cheat_time: usize) -> usize {
    let mut visited_set = HashSet::default();
    let mut visited = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(*start);
    while let Some(pos) = queue.pop_front() {
        if pos == *end {
            break;
        }
        for new_pos in pos.neighbors(map.len(), map[0].len()) {
            if map[new_pos.y()][new_pos.x()] && !visited_set.contains(&new_pos) {
                visited.push(new_pos);
                visited_set.insert(new_pos);
                queue.push_back(new_pos);
            }
        }
    }
    let mut ret = 0;
    for (steps, pos) in visited.iter().enumerate() {
        if let Some(shortcuts) = visited.get((steps + MINSAVE)..) {
            ret += shortcuts.iter().enumerate().filter(|(c, p)| pos.manhattan(p) <= cheat_time.min(*c)).count();
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 1);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 284);
    }
}
