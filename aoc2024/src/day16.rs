use log::debug;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use std::{collections::BinaryHeap, rc::Rc};

use crate::{
    pos::Dir,
    pos::{Coord, Pos},
};
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day16)]
fn parse(input: &str) -> (Vec<Vec<bool>>, Pos, Pos) {
    let mut start = Pos::new(0, 0);
    let mut end = Pos::new(0, 0);
    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            start = Pos::new(x, y);
                            true
                        }
                        'E' => {
                            end = Pos::new(x, y);
                            true
                        }
                        _ => panic!("Invalid character in input"),
                    })
                    .collect()
            })
            .collect(),
        start,
        end,
    )
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Steps {
    curr: Pos,
    dir: Dir,
    last: Option<Rc<Steps>>,
}
impl Steps {
    fn new(curr: Pos, dir: Dir) -> Self {
        Self { curr, last: None, dir }
    }
    fn add_step(&mut self, step: Pos, dir: Dir) {
        self.last = Some(Rc::new(self.clone()));
        self.curr = step;
        self.dir = dir;
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    pos: Pos,
    dir: Dir,
    steps: Steps,
    distance: usize,
}
impl State {
    fn new(cost: usize, pos: Pos, distance: usize, dir: Dir) -> Self {
        Self {
            cost,
            pos,
            distance,
            dir,
            steps: Steps::new(pos, dir),
        }
    }
    fn distcost(&self) -> usize {
        self.cost + (self.distance)
    }
    fn add_step(&mut self, step: Pos, dir: Dir) {
        self.steps.add_step(step, dir);
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distcost().cmp(&other.distcost())
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}
#[aoc(day16, part1)]
fn part1(input: &(Vec<Vec<bool>>, Pos, Pos)) -> usize {
    let mut queue = BinaryHeap::new();
    let (map, start, end) = input;
    let mut visited = vec![vec![(false, 0); map[0].len()]; map.len()];
    queue.push(State::new(0, *start, start.manhattan(end), Dir::Right));
    let mut vcpath = Vec::new();

    while let Some(State {
        cost,
        pos,
        distance: _,
        dir,
        steps: _,
    }) = queue.pop()
    {
        if pos == *end {
            vcpath.push(cost);
            continue;
        }
        if visited[pos.y()][pos.x()].0 {
            if visited[pos.y()][pos.x()].1 <= cost {
                continue;
            }
        }
        visited[pos.y()][pos.x()] = (true, cost);
        for nwdir in Dir::dirs() {
            let cost = match nwdir == dir {
                true => cost + 1,
                false => cost + 1001,
            };
            if let Some(newpos) = pos.dir(&nwdir, map.len(), map[0].len()) {
                if map[newpos.y()][newpos.x()] {
                    queue.push(State::new(cost, newpos, newpos.manhattan(end), nwdir));
                }
            }
        }
    }
    vcpath.iter().min().unwrap().to_owned()
}

#[aoc(day16, part2)]
fn part2(input: &(Vec<Vec<bool>>, Pos, Pos)) -> usize {
    let mut queue = BinaryHeap::new();
    let (map, start, end) = input;
    let mut visited = HashMap::default();
    queue.push(State::new(0, *start, start.manhattan(end), Dir::Right));
    let mut vcpath = Steps::new(*start, Dir::Right);
    let mut bestcost = usize::MAX;

    while let Some(State {
        cost,
        pos,
        distance: _,
        dir,
        steps,
    }) = queue.pop()
    {
        if cost > bestcost {
            break;
        }
        if let Some((oldvisit, mut oldsteps)) = visited.insert((pos, dir), (cost, vec![steps.clone()])) {
            if oldvisit < cost {
                let _ = visited.insert((pos, dir), (oldvisit, oldsteps));
                continue;
            }
            if oldvisit == cost {
                oldsteps.push(steps.clone());
                debug!("merged path at pos {:?}, ", pos,);
                let _ = visited.insert((pos, dir), (oldvisit, oldsteps));

                continue;
            }
        }
        if pos == *end {
            if cost <= bestcost {
                debug!("new best cost: {} {:?}", cost, pos);
                bestcost = cost;
                vcpath = steps;
            }
            continue;
        }
        for nwdir in Dir::dirs() {
            let cost = match nwdir == dir {
                true => cost + 1,
                false => cost + 1001,
            };
            if let Some(newpos) = pos.dir(&nwdir, map.len(), map[0].len()) {
                if map[newpos.y()][newpos.x()] {
                    let mut newstate = State::new(cost, newpos, newpos.manhattan(end), nwdir);
                    newstate.steps = steps.clone();
                    newstate.add_step(newpos, nwdir);
                    queue.push(newstate);
                }
            }
        }
    }
    // backtrack vcpath to find all the steps
    let mut steps = HashSet::default();
    let mut queue2 = vec![vcpath];
    while let Some(step) = queue2.pop() {
        steps.insert(step.curr);
        if let Some((_, vis_next)) = visited.get(&(step.curr, step.dir)) {
            for nextstep in vis_next {
                if let Some(last) = &nextstep.last {
                    queue2.push(last.as_ref().clone());
                }
            }
        }
    }
    steps.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 7036);
    }
    #[test]
    fn part2_example() {
        let mut queue = BinaryHeap::new();
        let (_map, start, end) = parse(TESTINPUT);
        let s1 = State::new(10, start, start.manhattan(&end), Dir::Up);
        let s2 = State::new(1210, start, start.manhattan(&end), Dir::Down);
        let s3 = State::new(11, start, start.manhattan(&end), Dir::Left);
        let s4 = State::new(110, start, start.manhattan(&end), Dir::Right);
        queue.push(s1);
        queue.push(s2);
        queue.push(s3);
        queue.push(s4);
        let first = queue.pop().unwrap();
        assert_eq!(first.cost, 10);
        assert_eq!(part2(&parse(TESTINPUT)), 45);
    }
}
