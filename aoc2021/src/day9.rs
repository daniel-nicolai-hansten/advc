use std::collections::{HashSet, VecDeque};

use crate::pos::{Coord, Pos};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}
fn find_lowest(map: &[Vec<u8>]) -> Vec<Pos> {
    let mut ret = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let pos = (x, y);
            let mut lowest = true;
            for neigh in pos.neighbors(map.len(), line.len()) {
                if c >= &map[neigh.y()][neigh.x()] {
                    lowest = false;
                    break;
                }
            }
            if lowest {
                ret.push(pos);
            }
        }
    }
    ret
}
#[aoc(day9, part1)]
fn part1(map: &[Vec<u8>]) -> u32 {
    find_lowest(map).len() as u32
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<u8>]) -> usize {
    let mut res = vec![];
    let lowpos = find_lowest(input);
    for pos in lowpos {
        let mut que = VecDeque::new();
        que.push_back((pos, 0));
        let mut visited = HashSet::new();
        while let Some((pos, dist)) = que.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            for neigh in pos.neighbors(input.len(), input[0].len()) {
                if input[neigh.y()][neigh.x()] > input[pos.y()][pos.x()] && input[neigh.y()][neigh.x()] != 9 {
                    que.push_back((neigh, dist + 1));
                }
            }
        }
        res.push(visited.len());
    }
    res.sort();
    res.iter().rev().take(3).fold(1, |acc, n| acc * n)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    #[test]
    fn part1_example() {
        let pos = (2, 5);
        assert_eq!(pos.down(5), None);
        //assert_eq!(part1(&parse(TESTINPUT)), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 1134);
    }
}
