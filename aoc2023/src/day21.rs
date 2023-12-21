use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day21)]
fn parse(input: &str) -> (Vec<Vec<Garden>>, Pos) {
    let mut map = vec![];
    let mut start = Pos { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Garden::Plot),
                '#' => row.push(Garden::Rock),
                'S' => {
                    row.push(Garden::Plot);
                    start = Pos { x, y };
                }
                _ => (),
            }
        }
        map.push(row);
    }
    (map, start)
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Garden {
    Plot,
    Rock,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn get_neighbor(&self) -> Vec<Pos> {
        let mut ret = vec![];
        if self.x > 0 {
            ret.push(Pos {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            ret.push(Pos {
                x: self.x,
                y: self.y - 1,
            });
        }
        ret.push(Pos {
            x: self.x + 1,
            y: self.y,
        });
        ret.push(Pos {
            x: self.x,
            y: self.y + 1,
        });
        ret
    }
}
#[aoc(day21, part1)]
fn part1(input: &(Vec<Vec<Garden>>, Pos)) -> usize {
    let (map, start) = input;
    // let mut visited: HashSet<Pos> = HashSet::new();
    let mut wq: VecDeque<Pos> = VecDeque::new();
    let mut nxq = Vec::new();
    wq.push_back(*start);
    for _ in 0..64 {
        while !wq.is_empty() {
            let pos = wq.pop_front().unwrap();
            for nxp in pos.get_neighbor() {
                if nxp.x < map[0].len() && nxp.y < map.len() && map[nxp.y][nxp.x] == Garden::Plot {
                    nxq.push(nxp);
                }
            }
        }
        nxq.sort_unstable();
        nxq.dedup();
        nxq.iter().for_each(|nxp| {
            wq.push_back(*nxp);
        });
        nxq.clear();
    }
    wq.len()
}

#[aoc(day21, part2)]
fn part2(input: &(Vec<Vec<Garden>>, Pos)) -> usize {
    let (map, start) = input;
    // let mut visited: HashSet<Pos> = HashSet::new();
    let mut wq: VecDeque<Pos> = VecDeque::new();
    let mut nxq = Vec::new();
    wq.push_back(*start);
    for _ in 0..64 {
        while !wq.is_empty() {
            let pos = wq.pop_front().unwrap();
            for nxp in pos.get_neighbor() {
                if map[nxp.y][nxp.x] == Garden::Plot {
                    nxq.push(nxp);
                }
            }
        }
        nxq.sort_unstable();
        nxq.dedup();
        nxq.iter().for_each(|nxp| {
            wq.push_back(*nxp);
        });
        nxq.clear();
    }
    wq.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 16);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
