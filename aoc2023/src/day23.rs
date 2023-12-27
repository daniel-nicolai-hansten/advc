use std::cmp::max;
use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<Vec<Terrain>> {
    let mut map = vec![];

    for line in input.lines() {
        let mut ln = vec![];
        for c in line.chars() {
            let trrn = match c {
                '.' => Terrain::Path,
                '#' => Terrain::Forest,
                '^' => Terrain::SteepSlope(Dir::Up),
                '>' => Terrain::SteepSlope(Dir::Right),
                'v' => Terrain::SteepSlope(Dir::Down),
                '<' => Terrain::SteepSlope(Dir::Left),
                _ => panic!("Unknown char"),
            };

            ln.push(trrn);
        }
        map.push(ln);
    }
    map
}
type Pos = (u8, u8);
trait Coord {
    fn x(&self) -> u8;
    fn y(&self) -> u8;
    fn xus(&self) -> usize;
    fn yus(&self) -> usize;
    fn pos(&self) -> Pos {
        (self.x(), self.y())
    }
    fn up(&self) -> Option<Pos> {
        if self.y() == 0 {
            None
        } else {
            Some((self.x(), self.y() - 1))
        }
    }
    fn down(&self) -> Option<Pos> {
        Some((self.x(), self.y() + 1))
    }
    fn left(&self) -> Option<Pos> {
        if self.x() == 0 {
            None
        } else {
            Some((self.x() - 1, self.y()))
        }
    }
    fn right(&self) -> Option<Pos> {
        Some((self.x() + 1, self.y()))
    }
    fn neighbors(&self) -> Vec<(Pos, Dir)> {
        let mut n = vec![];
        for p in [
            (self.up(), Dir::Up),
            (self.down(), Dir::Down),
            (self.left(), Dir::Left),
            (self.right(), Dir::Right),
        ] {
            if let (Some(p), dir) = p {
                n.push((p, dir));
            }
        }
        n
    }
    fn default() -> Pos {
        (0, 0)
    }
}
impl Coord for Pos {
    fn xus(&self) -> usize {
        self.0 as usize
    }
    fn yus(&self) -> usize {
        self.1 as usize
    }
    fn x(&self) -> u8 {
        self.0
    }
    fn y(&self) -> u8 {
        self.1
    }
}

struct RouteState {
    steps: u32,
    position: Pos,
    visited: Vec<Pos>,
}
#[aoc(day23, part1)]
fn part1(map: &[Vec<Terrain>]) -> u32 {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut results = vec![];
    'outer: for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == Terrain::Path {
                start = (x as u8, y as u8);
                break 'outer;
            }
        }
    }
    'outer: for (y, line) in map.iter().enumerate().rev() {
        for (x, c) in line.iter().enumerate().rev() {
            if *c == Terrain::Path {
                end = (x as u8, y as u8);
                break 'outer;
            }
        }
    }

    let visited = vec![start];
    let mut wq = Vec::new();
    wq.push(RouteState {
        steps: 0,
        position: start,
        visited,
    });
    while let Some(mut state) = wq.pop() {
        if state.position == end {
            results.push(state.steps);
            continue;
        }
        state.visited.push(state.position);
        let mut valid_next = vec![];
        for (n, _dir) in state.position.neighbors() {
            match (state.visited.contains(&n), map[n.yus()][n.xus()]) {
                (true, _) => {}
                (false, Terrain::Path) => valid_next.push(n),
                (false, Terrain::Forest) => {}
                (false, Terrain::SteepSlope(dir)) => match dir {
                    Dir::Up => {
                        if state.position.y() > n.y() {
                            valid_next.push(n);
                        }
                    }
                    Dir::Down => {
                        if state.position.y() < n.y() {
                            valid_next.push(n);
                        }
                    }
                    Dir::Left => {
                        if state.position.x() > n.x() {
                            valid_next.push(n);
                        }
                    }
                    Dir::Right => {
                        if state.position.x() < n.x() {
                            valid_next.push(n);
                        }
                    }
                },
            }
        }
        if !valid_next.is_empty() {
            let nextpos = valid_next.pop().unwrap();
            for pos in valid_next {
                wq.push(RouteState {
                    steps: state.steps + 1,
                    position: pos,
                    visited: state.visited.clone(),
                });
            }
            state.steps += 1;
            state.position = nextpos;
            wq.push(state);
        }
    }
    results.iter().max().unwrap().to_owned()
}

#[aoc(day23, part2)]
fn part2(map: &[Vec<Terrain>]) -> u32 {
    let mut max_steps = 0;
    if false {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut choke_points = HashMap::new();

        // let mut results = vec![];
        'outer: for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == Terrain::Path {
                    start = (x as u8, y as u8);
                    break 'outer;
                }
            }
        }
        'outer: for (y, line) in map.iter().enumerate().rev() {
            for (x, c) in line.iter().enumerate().rev() {
                if *c == Terrain::Path {
                    end = (x as u8, y as u8);
                    break 'outer;
                }
            }
        }

        let visited = vec![start];
        let mut wq = VecDeque::new();
        wq.push_back(RouteState {
            steps: 0,
            position: start,
            visited,
        });
        while let Some(mut state) = wq.pop_front() {
            if state.position == end {
                max_steps = max(state.steps, max_steps);
                println!("end at: {}  max:{}", state.steps, max_steps);
                continue;
            }
            state.visited.push(state.position);
            let mut valid_next = vec![];
            for (n, dir) in state.position.neighbors() {
                match (state.visited.contains(&n), map[n.yus()][n.xus()]) {
                    (true, _) => {}
                    (false, Terrain::Path | Terrain::SteepSlope(_)) => valid_next.push((n, dir)),
                    (false, Terrain::Forest) => {}
                }
            }
            if !valid_next.is_empty() {
                let (nextpos, dir) = valid_next.pop().unwrap();
                let mut branch = false;
                for (pos, _dir) in valid_next {
                    branch = true;
                    wq.push_back(RouteState {
                        steps: state.steps + 1,
                        position: pos,
                        visited: state.visited.clone(),
                    });
                }
                state.steps += 1;
                state.position = nextpos;
                if let true = branch {
                    wq.push_front(state);
                } else if let Some(steps) = choke_points.get(&(state.position, dir)) {
                    if *steps < state.steps {
                        choke_points.insert((state.position, dir), state.steps);
                        wq.push_front(state);
                    }
                } else {
                    wq.push_front(state);
                }
            }
        }
    }
    max_steps
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Terrain {
    Path,
    Forest,
    SteepSlope(Dir),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 94);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 154);
    }
}
