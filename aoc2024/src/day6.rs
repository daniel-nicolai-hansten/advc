use rustc_hash::FxHashSet as HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
type Pos = (usize, usize);
trait Coord {
    fn x(&self) -> usize;
    fn y(&self) -> usize;

    fn up(&self) -> Option<Pos> {
        if self.y() == 0 {
            None
        } else {
            Some((self.x(), self.y() - 1))
        }
    }
    fn down(&self, max: usize) -> Option<Pos> {
        if self.y() + 1 < max {
            Some((self.x(), self.y() + 1))
        } else {
            None
        }
    }

    fn left(&self) -> Option<Pos> {
        if self.x() == 0 {
            None
        } else {
            Some((self.x() - 1, self.y()))
        }
    }
    fn right(&self, max: usize) -> Option<Pos> {
        if self.x() + 1 < max {
            Some((self.x() + 1, self.y()))
        } else {
            None
        }
    }
}
impl Coord for Pos {
    fn x(&self) -> usize {
        self.0
    }
    fn y(&self) -> usize {
        self.1
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}
#[aoc_generator(day6)]
fn parse(input: &str) -> (Pos, Vec<Vec<bool>>) {
    let mut guard = (0, 0);
    let mut map = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut currentline = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => currentline.push(false),
                '#' => currentline.push(true),
                '^' => {
                    currentline.push(false);
                    guard = (j, i);
                }
                _ => panic!("Invalid character"),
            }
        }
        map.push(currentline);
    }
    (guard, map)
}

#[aoc(day6, part1)]
fn part1(input: &(Pos, Vec<Vec<bool>>)) -> usize {
    let (guard, map) = input;
    find_visited(map, guard).len()
}

#[aoc(day6, part2)]
fn part2(input: &(Pos, Vec<Vec<bool>>)) -> u32 {
    let (guard, map) = input;
    let visited = find_visited(map, guard);
    visited.par_iter().map(|&pos| if check(&map, guard, &(pos.x(), pos.y())) { 1 } else { 0 }).sum()
}

fn find_visited(map: &Vec<Vec<bool>>, guard: &Pos) -> HashSet<Pos> {
    let mut pos = *guard;
    let mut visited = HashSet::default();
    let mut dir = Dir::Up;
    loop {
        visited.insert(pos);
        let nextpos = match dir {
            Dir::Up => pos.up(),
            Dir::Down => pos.down(map.len()),
            Dir::Left => pos.left(),
            Dir::Right => pos.right(map[0].len()),
        };
        match nextpos {
            Some(nextpos) if map[nextpos.y()][nextpos.x()] => dir = dir.next(),
            Some(nextpos) => pos = nextpos,
            None => break,
        }
    }
    visited
}

fn check(map: &Vec<Vec<bool>>, guard: &Pos, obs: &Pos) -> bool {
    let mut pos = *guard;
    let mut visited = HashSet::default();
    let mut dir = Dir::Up;
    let mut loopdtct = false;
    loop {
        if !visited.insert((pos, dir)) {
            loopdtct = true;
            break;
        };
        let nextpos = match dir {
            Dir::Up => pos.up(),
            Dir::Down => pos.down(map.len()),
            Dir::Left => pos.left(),
            Dir::Right => pos.right(map[0].len()),
        };
        match nextpos {
            Some(nextpos) if map[nextpos.y()][nextpos.x()] => dir = dir.next(),
            Some(nextpos) if nextpos == *obs => dir = dir.next(),
            Some(nextpos) => pos = nextpos,
            None => break,
        }
    }
    loopdtct
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT1)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT1)), 6);
    }
}
