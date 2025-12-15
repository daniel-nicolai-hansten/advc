use std::collections::{BinaryHeap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::pos::{Coord, Pos};
#[aoc_generator(day15, part1)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut ln = Vec::new();
        for c in line.chars() {
            ln.push(c.to_digit(10).unwrap());
        }
        map.push(ln);
    }
    map
}
#[aoc_generator(day15, part2)]
fn parse2(input: &str) -> Vec<Vec<u32>> {
    let mut map = Vec::new();
    let clamp = |x| if x < 10 { x } else { x % 9 };
    for y in 0..5 {
        for line in input.lines() {
            let mut ln = Vec::new();
            for x in 0..5 {
                for c in line.chars() {
                    ln.push(clamp(c.to_digit(10).unwrap() + x + y));
                }
            }
            map.push(ln);
        }
    }
    // for ln in map.iter() {
    //     for n in ln.iter() {
    //         print!("{}", n);
    //     }
    //     println!();
    // }
    map
}
struct State(Pos, usize);
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = other.1 + self.0.manhattan(&(0, 0));
        let b = self.1 + other.0.manhattan(&(0, 0));
        a.cmp(&b)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
impl Eq for State {}

#[aoc(day15, part1)]
fn part1(map: &[Vec<u32>]) -> usize {
    let start = (0, 0);
    let max_x = map[0].len();
    let max_y = map.len();
    let end = (max_x - 1, max_y - 1);
    let mut visited = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State(start, 0));
    while let Some(State(pos, risk)) = queue.pop() {
        if pos == end {
            return risk;
        }
        if !visited.insert(pos) {
            continue;
        }
        for next in pos.neighbors(max_y, max_x) {
            queue.push(State(next, risk + map[next.y()][next.x()] as usize));
        }
    }
    0
}

#[aoc(day15, part2)]
fn part2(map: &[Vec<u32>]) -> usize {
    let start = (0, 0);
    let max_x = map[0].len();
    let max_y = map.len();
    let end = (max_x - 1, max_y - 1);
    let mut visited = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State(start, 0));
    while let Some(State(pos, risk)) = queue.pop() {
        if pos == end {
            return risk;
        }
        if !visited.insert(pos) {
            continue;
        }
        for next in pos.neighbors(max_y, max_x) {
            queue.push(State(next, risk + map[next.y()][next.x()] as usize));
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 40);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(TESTINPUT)), 315);
    }
}
