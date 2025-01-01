use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{max, min};

#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    input.lines().map(|l| format!("{}\n", l.trim())).collect()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let mut pos_list = vec![];
    for line in input.lines() {
        let pair: Vec<&str> = line.split(" -> ").collect();
        let p1: Pos = pair[0].into();
        let p2: Pos = pair[1].into();
        if p1.x == p2.x {
            let x = p1.x;
            let y_min = min(p1.y, p2.y);
            let y_max = max(p1.y, p2.y);
            for y in y_min..=y_max {
                pos_list.push(Pos { y, x });
            }
        } else if p1.y == p2.y {
            let y = p1.y;
            let x_min = min(p1.x, p2.x);
            let x_max = max(p1.x, p2.x);
            for x in x_min..=x_max {
                pos_list.push(Pos { y, x });
            }
        }
    }
    pos_list.sort_unstable();
    let mut pos_list2 = vec![];
    let mut lastpos = None;
    for pos in &pos_list {
        if Some(*pos) == lastpos {
            pos_list2.push(*pos);
        }
        lastpos = Some(*pos);
    }
    pos_list2.sort_unstable();
    pos_list2.dedup();
    pos_list2.len()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    let mut pos_list = vec![];
    for line in input.lines() {
        let pair: Vec<&str> = line.split(" -> ").collect();
        let p1: Pos = pair[0].into();
        let p2: Pos = pair[1].into();
        if p1.x == p2.x {
            let x = p1.x;
            let y_min = min(p1.y, p2.y);
            let y_max = max(p1.y, p2.y);
            for y in y_min..=y_max {
                pos_list.push(Pos { y, x });
            }
        } else if p1.y == p2.y {
            let y = p1.y;
            let x_min = min(p1.x, p2.x);
            let x_max = max(p1.x, p2.x);
            for x in x_min..=x_max {
                pos_list.push(Pos { y, x });
            }
        } else if p1.x > p2.x && p1.y < p2.y {
            for (i, x) in (p2.x..=p1.x).enumerate() {
                let y = p2.y - i;
                pos_list.push(Pos { y, x });
            }
        } else if p1.x < p2.x && p1.y > p2.y {
            for (i, x) in (p1.x..=p2.x).enumerate() {
                let y = p1.y - i;
                pos_list.push(Pos { y, x });
            }
        } else if p1.x > p2.x && p1.y > p2.y {
            for (i, x) in (p2.x..=p1.x).enumerate() {
                let y = p2.y + i;
                pos_list.push(Pos { y, x });
            }
        } else if p1.x < p2.x && p1.y < p2.y {
            for (i, x) in (p1.x..=p2.x).enumerate() {
                let y = p1.y + i;
                pos_list.push(Pos { y, x });
            }
        }
    }
    pos_list.sort_unstable();
    let mut pos_list2 = vec![];
    let mut lastpos = None;
    for pos in &pos_list {
        if Some(*pos) == lastpos {
            pos_list2.push(*pos);
        }
        lastpos = Some(*pos);
    }
    pos_list2.sort_unstable();
    pos_list2.dedup();
    pos_list2.len()
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}
impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        let splits: Vec<&str> = s.split(",").collect();
        let x = usize::from_str_radix(splits[0], 10).unwrap();
        let y = usize::from_str_radix(splits[1], 10).unwrap();
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 12);
    }
}
