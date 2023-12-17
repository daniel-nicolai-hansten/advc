use aoc_runner_derive::{aoc, aoc_generator};

use rustc_hash::FxHashMap;
use std::{
    cmp::{max, min, Reverse},
    collections::{BinaryHeap, VecDeque},
};
#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut ret = vec![];
    for line in input.lines() {
        let mut tmpvec = vec![];
        for c in line.chars() {
            tmpvec.push(u32::from_str_radix(&c.to_string(), 10).unwrap());
        }
        ret.push(tmpvec)
    }
    ret
}

#[aoc(day17, part1)]
fn part1(map: &[Vec<u32>]) -> u32 {
    let startpos = Pos { x: 0, y: 0 };
    let end = Pos {
        x: map[0].len() - 1,
        y: map.len() - 1,
    };
    let mut currentbest = u32::MAX;
    let mut wq = VecDeque::new();
    let mut visited = FxHashMap::default();
    wq.push_back((startpos, Dir::East, 0, 1));
    loop {
        for _ in 0..wq.len() {
            if let Some((pos, dir, heatloss, dircount)) = wq.pop_front() {
                if let Some(heatloss_last) = visited.get(&(pos, dir, dircount)) {
                    if *heatloss_last <= heatloss {
                        continue;
                    } else {
                        visited.insert((pos, dir, dircount), heatloss);
                    }
                } else {
                    visited.insert((pos, dir, dircount), heatloss);
                }
                if heatloss > currentbest {
                    continue;
                }

                for (p, newdir, dircnt) in valid_next_move_p1(map, dir, pos, dircount) {
                    let new_heatloss = heatloss + map[p.y][p.x];
                    if p == end {
                        currentbest = min(currentbest, new_heatloss);
                    } else {
                        wq.push_back((p, newdir, new_heatloss, dircnt));
                    }
                }
            }
        }
        if wq.is_empty() {
            break;
        }
    }
    currentbest
}

#[aoc(day17, part2)]
fn part2(map: &[Vec<u32>]) -> u32 {
    let startpos = Pos { x: 0, y: 0 };
    let end = Pos {
        x: map[0].len() - 1,
        y: map.len() - 1,
    };
    let mut currentbest = u32::MAX;
    let mut visited = FxHashMap::default();
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, (startpos, Dir::East, 0, 1))));
    pq.push(Reverse((0, (startpos, Dir::South, 0, 1))));
    loop {
        if let Some(Reverse((qpri, (pos, dir, heatloss, dircount)))) = pq.pop() {
            if qpri > currentbest {
                break;
            }
            if let Some(heatloss_last) = visited.get(&(pos, dir, dircount)) {
                if *heatloss_last < heatloss {
                    continue;
                } else {
                    visited.insert((pos, dir, dircount), heatloss);
                }
            } else {
                visited.insert((pos, dir, dircount), heatloss);
            }
            if heatloss > currentbest {
                continue;
            }
            for (p, newdir, dircnt) in valid_next_move_p2(map, dir, pos, dircount) {
                let new_heatloss = heatloss + map[p.y][p.x];
                if p == end && dircnt >= 4 {
                    currentbest = min(currentbest, new_heatloss);
                } else {
                    match visited.insert((p, newdir, dircnt), new_heatloss) {
                        None => {
                            let pri = new_heatloss + (p.distance(&end) as u32 * 2)
                                as u32;
                            pq.push(Reverse((pri, (p, newdir, new_heatloss, dircnt))));
                        },
                        Some(hl) => {visited.insert((p, newdir, dircnt), hl);},
                    }
                }
            }
        } else {
            break;
        }
    }
    currentbest
}
fn valid_next_move_p1(map: &[Vec<u32>], dir: Dir, pos: Pos, dircount: u32) -> Vec<(Pos, Dir, u32)> {
    let mut ret = vec![];
    let max_y = map.len();
    let max_x = map[0].len();
    match dir {
        Dir::North | Dir::South => {
            ret.push((pos.east(), Dir::East, 1));
            ret.push((pos.west(), Dir::West, 1));
        }
        Dir::West | Dir::East => {
            ret.push((pos.north(), Dir::North, 1));
            ret.push((pos.south(), Dir::South, 1));
        }
    }
    if dircount < 3 {
        match dir {
            Dir::North => ret.push((pos.north(), dir, dircount + 1)),
            Dir::South => ret.push((pos.south(), dir, dircount + 1)),
            Dir::West => ret.push((pos.west(), dir, dircount + 1)),
            Dir::East => ret.push((pos.east(), dir, dircount + 1)),
        }
    }
    ret.into_iter()
        .filter(|(p, _, _)| *p != pos && p.x < max_x && p.y < max_y)
        .collect()
}
fn valid_next_move_p2(map: &[Vec<u32>], dir: Dir, pos: Pos, dircount: u32) -> Vec<(Pos, Dir, u32)> {
    let mut ret = vec![];
    let max_y = map.len();
    let max_x = map[0].len();
    if dircount >= 4 {
        match dir {
            Dir::North | Dir::South => {
                ret.push((pos.east(), Dir::East, 1));
                ret.push((pos.west(), Dir::West, 1));
            }
            Dir::West | Dir::East => {
                ret.push((pos.north(), Dir::North, 1));
                ret.push((pos.south(), Dir::South, 1));
            }
        }
    }
    if dircount < 10 {
        match dir {
            Dir::North => ret.push((pos.north(), dir, dircount + 1)),
            Dir::South => ret.push((pos.south(), dir, dircount + 1)),
            Dir::West => ret.push((pos.west(), dir, dircount + 1)),
            Dir::East => ret.push((pos.east(), dir, dircount + 1)),
        }
    }
    ret.into_iter()
        .filter(|(p, _, _)| *p != pos && p.x < max_x && p.y < max_y)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    North,
    West,
    South,
    East,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
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
    fn distance(&self, other: &Pos) -> usize {
        let x_diff = max(self.x, other.x) - min(self.x, other.x);
        let y_diff = max(self.y, other.y) - min(self.y, other.y);
        x_diff + y_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    const TESTINPUT2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 102);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 94);
    }
    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(TESTINPUT2)), 71);
    }
}
