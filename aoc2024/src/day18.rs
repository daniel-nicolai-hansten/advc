use rustc_hash::FxHashSet as HashSet;
use std::{collections::BinaryHeap, rc::Rc};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::pos::{Coord, Pos};
#[cfg(not(test))]
const GRID_SIZE: usize = 70 + 1;
#[cfg(not(test))]
const FALLCNT: usize = 1024;
#[cfg(test)]
const GRID_SIZE: usize = 6 + 1;
#[cfg(test)]
const FALLCNT: usize = 12;

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Pos> {
    let (_i, o) = prse(input).unwrap();
    o.iter().map(|(x, y)| (*x as usize, *y as usize)).collect()
}
fn prse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(newline, separated_pair(complete::u32, tag(","), complete::u32))(input)
}

#[aoc(day18, part1)]
fn part1(input: &[Pos]) -> usize {
    let mut map = [[true; GRID_SIZE]; GRID_SIZE];
    let start = Pos::new(0, 0);
    let end = Pos::new(GRID_SIZE - 1, GRID_SIZE - 1);
    for (cnt, px) in input.iter().enumerate() {
        if cnt == FALLCNT {
            break;
        }
        map[px.1 as usize][px.0 as usize] = false;
    }

    find_exit(&map, start, end).unwrap().0
}

#[aoc(day18, part2)]
fn part2(input: &[Pos]) -> String {
    let mut res = (0, 0);
    let mut map = [[true; GRID_SIZE]; GRID_SIZE];
    let start = Pos::new(0, 0);
    let end = Pos::new(GRID_SIZE - 1, GRID_SIZE - 1);
    let mut cursteps = Vec::new();
    for (idx, px) in input.iter().enumerate() {
        map[px.1 as usize][px.0 as usize] = false;
        if idx < FALLCNT {
            continue;
        }
        if cursteps.contains(px) || cursteps.is_empty() {
            match find_exit(&map, start, end) {
                None => {
                    res = *px;
                    break;
                }
                Some((_, stps)) => {
                    cursteps = stps;
                }
            }
        }
    }
    format!("{},{}", res.0, res.1)
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Step {
    pos: Pos,
    last: Option<Rc<Step>>,
    steps: usize,
}
impl Step {
    fn distcost(&self) -> usize {
        self.steps.saturating_sub(self.pos.0 + self.pos.1)
    }
    fn add(&self, pos: Pos) -> Self {
        Self {
            pos,
            last: Some(Rc::new(self.clone())),
            steps: self.steps + 1,
        }
    }
}
impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distcost().cmp(&self.distcost())
    }
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_exit(map: &[[bool; GRID_SIZE]; GRID_SIZE], start: Pos, end: Pos) -> Option<(usize, Vec<Pos>)> {
    let mut res = None;
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::default();
    let backtrace = |steps: Step| {
        let mut ret = vec![steps.pos];
        let mut curstep = &steps.last;
        while let Some(step) = curstep {
            ret.push(step.pos);
            curstep = &step.last;
        }
        ret
    };
    queue.push(Step {
        pos: start,
        last: None,
        steps: 0,
    });
    while let Some(last_st) = queue.pop() {
        let pos = last_st.pos;
        if pos == end {
            res = Some((last_st.steps, backtrace(last_st)));
            break;
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        for new_pos in pos.neighbors(GRID_SIZE, GRID_SIZE) {
            if map[new_pos.y()][new_pos.x()] {
                queue.push(last_st.add(new_pos));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    #[test]
    fn part1_example() {
        let mut queue = BinaryHeap::new();
        queue.push(Step {
            pos: (0, 0),
            last: None,
            steps: 4,
        });
        queue.push(Step {
            pos: (0, 0),
            last: None,
            steps: 1,
        });
        queue.push(Step {
            pos: (0, 0),
            last: None,
            steps: 6,
        });
        queue.push(Step {
            pos: (0, 0),
            last: None,
            steps: 14,
        });
        assert_eq!(
            queue.pop().unwrap(),
            Step {
                pos: (0, 0),
                last: None,
                steps: 1
            }
        );
        assert_eq!(part1(&parse(TESTINPUT)), 22);
    }

    #[test]
    fn part2_example() {
        // assert_eq!(part2(&parse(TESTINPUT)), "6,1");
    }
}
