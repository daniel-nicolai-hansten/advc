use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct InfPos {
    x: usize,
    y: usize,
    mapx: i32,
    mapy: i32,
}
impl InfPos {
    fn get_neighbor(&self, ymax: usize, xmax: usize) -> Vec<InfPos> {
        let mut ret = vec![];
        if self.x > 0 {
            ret.push(InfPos {
                x: self.x - 1,
                y: self.y,
                mapx: self.mapx,
                mapy: self.mapy,
            });
        } else {
            ret.push(InfPos {
                x: self.x + xmax - 1,
                y: self.y,
                mapx: self.mapx - 1,
                mapy: self.mapy,
            });
        }
        if self.y > 0 {
            ret.push(InfPos {
                x: self.x,
                y: self.y - 1,
                mapx: self.mapx,
                mapy: self.mapy,
            });
        } else {
            ret.push(InfPos {
                x: self.x,
                y: self.y + ymax - 1,
                mapx: self.mapx,
                mapy: self.mapy - 1,
            });
        }
        if self.x + 1 < xmax {
            ret.push(InfPos {
                x: self.x + 1,
                y: self.y,
                mapx: self.mapx,
                mapy: self.mapy,
            });
        } else {
            ret.push(InfPos {
                x: (self.x + 1) % xmax,
                y: self.y,
                mapx: self.mapx + 1,
                mapy: self.mapy,
            });
        }
        if self.y + 1 < ymax {
            ret.push(InfPos {
                x: self.x,
                y: self.y + 1,
                mapx: self.mapx,
                mapy: self.mapy,
            });
        } else {
            ret.push(InfPos {
                x: self.x,
                y: (self.y + 1) % ymax,
                mapx: self.mapx,
                mapy: self.mapy + 1,
            });
        }
        ret
    }
}
#[aoc(day21, part1)]
fn part1(input: &(Vec<Vec<Garden>>, Pos)) -> usize {
    #[cfg(test)]
    let steps = 6;
    #[cfg(not(test))]
    let steps = 64;
    let (map, start) = input;
    // let mut visited: HashSet<Pos> = HashSet::new();
    let mut wq: VecDeque<Pos> = VecDeque::new();
    let mut nxq = Vec::new();
    wq.push_back(*start);
    for _ in 0..steps {
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
use num::integer::Roots;
use crate::day9::{predict, Dir};
#[aoc(day21, part2)]
fn part2(input: &(Vec<Vec<Garden>>, Pos)) ->i64 {
    let mut fields = vec![];
    let (map, st) = input;
    let start = InfPos {
        x: st.x,
        y: st.y,
        mapx: 0,
        mapy: 0,
    };
    let mut visited: HashSet<InfPos> = HashSet::new();
    let (ymax, xmax) = (map.len(), map[0].len());
    let mut wq: VecDeque<InfPos> = VecDeque::new();
    let mut nxq = Vec::new();
    // let trgt = InfPos {

    // }
    wq.push_back(start);
    // visited.insert(start);
    'outer: for i in 0..460{
        while !wq.is_empty() {
            let pos = wq.pop_front().unwrap();
            // if pos == trgt {
            //     break 'outer;
            // }
            for nxp in pos.get_neighbor(ymax, xmax) {
                if map[nxp.y % ymax][nxp.x % xmax] == Garden::Plot && !visited.contains(&nxp) {
                    // visited.insert(nxp);
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
        if i > 65 && (i - 65) % 131 == 0 {
            println!("{i}: {} , ", wq.len());
            fields.push(wq.len());
        }
    }
    let mut pattern:Vec<i64> = fields.iter().map(|n| *n as i64).collect();
    for _ in 0..202_300 {
        let len = pattern.len() -3;
        let num = predict(&pattern[len..], &Dir::Fwd);
        println!("{num}");
        pattern.push(num);
    }
    // let mut diffs = vec![];
    // for (f1, f2) in fields.iter().tuple_windows() {
    //     let diff = f2.abs_diff(*f1);
    //     diffs.push(diff);
    //     // println!("{diff_sq}, {f2} ");
    // }
    // diffs
    //     .iter()
    //     .tuple_windows()
    //     .map(|(diff1, diff2)| diff1.abs_diff(*diff2))
    //     .for_each(|n| print!("{n}  "));
    pattern[202_300]
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 0);
    }
}
