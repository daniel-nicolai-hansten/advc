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
struct Octo {
    pos: Pos,
    val: u32,
}
impl Octo {
    fn dist(&self, other: &Self) -> usize {
        self.pos.manhattan(&other.pos)
    }
    fn add(&mut self) -> bool {
        self.val += 1;
        self.val == 10
    }
}
#[aoc(day11, part1)]
fn part1(input: &[Octo]) -> String {
let mut flashing = VecDeque::new();
let mut octopuses = input.clone();
for octo in octopuses.iter_mut() {
    if octo.add() {
        flashing.push_back(octo.clone());
    }
}
while let Some(flash) = flashing.pop_front() {
   octopuses.iter_mut().filter(||) 
}
        todo!()
}

#[aoc(day11, part2)]
fn part2(input: &[Octo]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
