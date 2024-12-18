use rustc_hash::FxHashSet as HashSet;
use std::{collections::VecDeque, rc::Rc};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::pos::{Coord, Pos};
const GRID_SIZE: usize = 70 + 1;
#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Pos> {
    let (_i, o) = prse(input).unwrap();
    o.iter().map(|(x, y)| (*x as usize, *y as usize)).collect()
}
fn prse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(newline, separated_pair(complete::u32, tag(","), complete::u32))(input)
}

#[aoc(day18, part1)]
fn part1(input: &[Pos]) -> u32 {
    let mut map = [[true; GRID_SIZE]; GRID_SIZE];
    let start = Pos::new(0, 0);
    let end = Pos::new(GRID_SIZE - 1, GRID_SIZE - 1);
    let mut bytecount = 0;
    for px in input {
        map[px.1 as usize][px.0 as usize] = false;
        bytecount += 1;
        if bytecount == 1024 {
            break;
        }
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
        if idx < 1024 {
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
struct Step {
    pos: Pos,
    last: Option<Rc<Step>>,
}

fn find_exit(map: &[[bool; GRID_SIZE]; GRID_SIZE], start: Pos, end: Pos) -> Option<(u32, Vec<Pos>)> {
    let mut res = None;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::default();
    let backtrace = |steps: Rc<Step>| {
        let mut ret = vec![steps.pos];
        let mut curstep = &steps.last;
        while let Some(step) = curstep {
            ret.push(step.pos);
            curstep = &step.last;
        }
        ret
    };
    queue.push_back((0, Rc::new(Step { pos: start, last: None })));
    while let Some((steps, last_st)) = queue.pop_front() {
        let pos = last_st.pos;
        if pos == end {
            res = Some((steps, backtrace(last_st)));
            break;
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        for new_pos in pos.neighbors(GRID_SIZE, GRID_SIZE) {
            if map[new_pos.y()][new_pos.x()] {
                let new_step = Rc::new(Step {
                    pos: new_pos,
                    last: Some(last_st.clone()),
                });
                queue.push_back((steps + 1, new_step));
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
        assert_eq!(part1(&parse(TESTINPUT)), 22);
    }

    #[test]
    fn part2_example() {
        // assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
