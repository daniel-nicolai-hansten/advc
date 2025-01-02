use std::collections::VecDeque;

use crate::pos::Coord;
use crate::pos::Pos;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Octo> {
    let mut ret = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            ret.push(Octo {
                pos: (x, y),
                val: c.to_digit(10).unwrap(),
            })
        }
    }
    ret
}
#[derive(Clone, Debug)]
struct Octo {
    pos: Pos,
    val: u32,
}
impl Octo {
    fn dist(&self, other: &Self) -> usize {
        std::cmp::max(self.pos.x().abs_diff(other.pos.x()), self.pos.y().abs_diff(other.pos.y())) as usize
    }
    fn add(&mut self) -> bool {
        self.val += 1;
        self.val == 10
    }
    fn reset(&mut self) {
        self.val = 0;
    }
}
#[aoc(day11, part1)]
fn part1(input: &[Octo]) -> usize {
    let mut flashing = VecDeque::new();
    let mut tot_flash = 0;
    let mut octopuses = input.to_vec();
    for _ in 0..100 {
        for octo in octopuses.iter_mut() {
            if octo.add() {
                flashing.push_back(octo.clone());
            }
        }
        while let Some(flash) = flashing.pop_front() {
            tot_flash += 1;
            octopuses.iter_mut().filter(|octo| octo.dist(&flash) <= 1).for_each(|octo| {
                if octo.add() {
                    flashing.push_back(octo.clone());
                }
            });
        }
        octopuses.iter_mut().filter(|octo| octo.val > 9).for_each(|octo| octo.reset());
    }
    tot_flash
}

#[aoc(day11, part2)]
fn part2(input: &[Octo]) -> usize {
    let mut flashing = VecDeque::new();
    let mut ret = 0;
    let mut octopuses = input.to_vec();
    for i in 1..1000 {
        for octo in octopuses.iter_mut() {
            if octo.add() {
                flashing.push_back(octo.clone());
            }
        }
        while let Some(flash) = flashing.pop_front() {
            octopuses.iter_mut().filter(|octo| octo.dist(&flash) <= 1).for_each(|octo| {
                if octo.add() {
                    flashing.push_back(octo.clone());
                }
            });
        }
        if octopuses.iter_mut().filter(|octo| octo.val > 9).count() == octopuses.len() {
            ret = i;
            break;
        }
        octopuses.iter_mut().filter(|octo| octo.val > 9).for_each(|octo| octo.reset());
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 1656);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 195);
    }
}
