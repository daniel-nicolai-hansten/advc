use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::collections::{HashSet, VecDeque};
#[aoc_generator(day16)]
fn parse(input: &str) -> Vec<Vec<char>> {
    let mut ret = vec![];
    for line in input.lines() {
        let mut tmpvec = vec![];
        for c in line.chars() {
            tmpvec.push(c);
        }
        ret.push(tmpvec)
    }
    ret
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    fn north(&self) -> Pos {
        let y = if self.y > 0 { self.y - 1 } else { self.y };
        Pos { y, x: self.x }
    }
    fn south(&self) -> Pos {
        Pos {
            y: self.y + 1,
            x: self.x,
        }
    }
    fn east(&self) -> Pos {
        Pos {
            y: self.y,
            x: self.x + 1,
        }
    }
    fn west(&self) -> Pos {
        let x = if self.x > 0 { self.x - 1 } else { self.x };
        Pos { y: self.y, x }
    }
}

#[aoc(day16, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    let startpos = Pos { x: 0, y: 0 };
    let startdir = Dir::Right;
    beam_energy(input, startpos, startdir)
}

fn beam_energy(input: &[Vec<char>], startpos: Pos, startdir: Dir) -> usize {
    let max_x = input[0].len() as u8;
    let max_y = input.len() as u8;
    let mut visited = FxHashSet::default();
    let mut beamque = VecDeque::new();
    beamque.push_back((startpos, startdir));

    while !beamque.is_empty() {
        let (mut pos, mut dir) = beamque.pop_front().unwrap();
        'inner: loop {
            if !visited.insert((pos, dir)) {
                break 'inner;
            }
            match input[pos.y as usize][pos.x as usize] {
                '/' => {
                    dir = match dir {
                        Dir::Down => Dir::Left,
                        Dir::Up => Dir::Right,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                    }
                }
                '\\' => {
                    dir = match dir {
                        Dir::Down => Dir::Right,
                        Dir::Up => Dir::Left,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    }
                }
                '-' => match dir {
                    Dir::Down | Dir::Up => {
                        let beam1 = (pos.clone(), Dir::Left);
                        let beam2 = (pos.clone(), Dir::Right);
                        if !visited.contains(&beam1) {
                            beamque.push_back(beam1);
                        }
                        if !visited.contains(&beam2) {
                            beamque.push_back(beam2);
                        }
                        break 'inner;
                    }
                    Dir::Left | Dir::Right => (),
                },
                '|' => match dir {
                    Dir::Left | Dir::Right => {
                        let beam1 = (pos.clone(), Dir::Up);
                        let beam2 = (pos.clone(), Dir::Down);
                        if !visited.contains(&beam1) {
                            beamque.push_back(beam1);
                        }
                        if !visited.contains(&beam2) {
                            beamque.push_back(beam2);
                        }
                        break 'inner;
                    }
                    Dir::Down | Dir::Up => (),
                },
                _ => (),
            }
            match dir {
                Dir::Down if pos.y < max_y - 1 => pos = pos.south(),
                Dir::Up if pos.y > 0 => pos = pos.north(),
                Dir::Left if pos.x > 0 => pos = pos.west(),
                Dir::Right if pos.x < max_x - 1 => pos = pos.east(),
                _ => {
                    break 'inner;
                }
            }
        }
    }
    visited
        .iter()
        .map(|(p, _d)| p)
        .sorted()
        .dedup()
        .fold(0, |acc, _p| acc + 1)
}

#[allow(dead_code)]
fn map_printer(map: &[Vec<char>], visited: &HashSet<(Pos, Dir)>) {
    for (yn, line) in map.iter().enumerate() {
        for (xn, c) in line.iter().enumerate() {
            let (x, y) = (xn as u8, yn as u8);
            let sym = match (
                visited.contains(&(Pos { x, y }, Dir::Down)),
                visited.contains(&(Pos { x, y }, Dir::Up)),
                visited.contains(&(Pos { x, y }, Dir::Left)),
                visited.contains(&(Pos { x, y }, Dir::Right)),
            ) {
                (true, _, _, _) => 'v',
                (_, true, _, _) => '^',
                (_, _, true, _) => '<',
                (_, _, _, true) => '>',
                _ => *c,
            };
            print!("{sym}");
        }
        println!();
    }
}
#[aoc(day16, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let mut wq = vec![];
    let max_y = input.len() as u8;
    let max_x = input[0].len() as u8;
    for x in 0..max_x {
        wq.push((Pos { y: 0, x }, Dir::Down));
        wq.push((Pos { y: max_y - 1 as u8, x }, Dir::Up));
    }
    for y in 0..max_y {
        wq.push((Pos { y, x: 0 }, Dir::Right));
        wq.push((Pos { y, x: max_x - 1 as u8 }, Dir::Left));
    }
    wq.par_iter().map(|(p, d)| beam_energy(input, *p, *d)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 51);
    }
}
