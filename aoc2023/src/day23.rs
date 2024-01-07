use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;
use std::thread::sleep;

#[aoc_generator(day23)]
fn parse(input: &str) -> (Vec<Vec<Terrain>>, Vec<Pos>, Pos, Pos) {
    let mut map = vec![];
    let (mut start, mut end) = ((0, 0), (0, 0));
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
                _ => panic!("Invalid char"),
            };

            ln.push(trrn);
        }
        map.push(ln);
    }
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
    let mapptr = &map;
    let mut nodes: Vec<Pos> = map
        .par_iter()
        .enumerate()
        .flat_map_iter(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                let pos = (x as u8, y as u8);
                if &Terrain::Path == c {
                    let neighbors = pos
                        .neighbors(mapptr)
                        .iter()
                        .filter(|((nx, ny), _)| {
                            matches!(
                                mapptr[*ny as usize][*nx as usize],
                                Terrain::Path | Terrain::SteepSlope(_)
                            )
                        })
                        .count();
                    if neighbors > 2 {
                        Some(pos)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect();
    nodes.push(start);
    nodes.push(end);
    (map, nodes, start, end)
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
    fn down(&self, max: usize) -> Option<Pos> {
        if self.yus() + 1 < max {
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
        if self.xus() + 1 < max {
            Some((self.x() + 1, self.y()))
        } else {
            None
        }
    }
    fn neighbors(&self, map: &[Vec<Terrain>]) -> Vec<(Pos, Dir)> {
        let mut n = vec![];
        for p in [
            (self.up(), Dir::Up),
            (self.down(map.len()), Dir::Down),
            (self.left(), Dir::Left),
            (self.right(map[0].len()), Dir::Right),
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
    steps: usize,
    position: NodeIndex,
    visited: u64,
}

#[aoc(day23, part1)]
fn part1(input: &(Vec<Vec<Terrain>>, Vec<Pos>, Pos, Pos)) -> usize {
    let (map, nodes, start, end) = input;
    let mut graph: Graph<(u8, u8), usize> = Graph::new();
    let mut node_idx = HashMap::default();
    for node in nodes {
        let idx = graph.add_node(node.pos());
        node_idx.insert(node.pos(), idx);
    }
    for node in nodes {
        let neighbors = find_neighbor_node1(*node, &map, &nodes);

        for (n, steps) in neighbors {
            let idx = node_idx.get(&node.pos()).unwrap();
            let idx2 = node_idx.get(&n).unwrap();
            graph.update_edge(*idx, *idx2, steps);
        }
    }
    find_longest_path(start, end, graph)
}

#[aoc(day23, part2)]
fn part2(input: &(Vec<Vec<Terrain>>, Vec<Pos>, Pos, Pos)) -> usize {
    let (map, nodes, start, end) = input;
    let mut graph: Graph<(u8, u8), usize> = Graph::new();
    let mut node_idx = HashMap::default();
    for node in nodes {
        let idx = graph.add_node(node.pos());
        node_idx.insert(node.pos(), idx);
    }
    for node in nodes {
        let neighbors = find_neighbor_node2(*node, &map, &nodes);

        for (n, steps) in neighbors {
            let idx = node_idx.get(&node.pos()).unwrap();
            let idx2 = node_idx.get(&n).unwrap();
            graph.update_edge(*idx, *idx2, steps);
        }
    }
    find_longest_path(start, end, graph)
}
enum WqHandler {
    Add(RouteState),
    Pop(std::sync::mpsc::Sender<WqState>),
    End(RouteState),
}
enum WqState {
    Wrk(RouteState),
    Done,
    Retry,
}
use std::thread;
fn find_longest_path(start: &Pos, end: &Pos, graph: Graph<(u8, u8), usize>) -> usize {
    let start = graph.node_indices().find(|idx| graph[*idx] == *start).unwrap();
    let end = graph.node_indices().find(|idx| graph[*idx] == *end).unwrap();
    let startconf = RouteState {
        steps: 0,
        position: start,
        visited: 0 | (1 << start.index()),
    };
    let (tx, rx) = std::sync::mpsc::channel::<WqHandler>();
    let threads = thread::available_parallelism().unwrap();
    let res = thread::scope(|s| {
        let max_steps = s.spawn(move || {
            let mut max_steps = 0;
            let mut wq = Vec::new();
            wq.push(startconf);
            while let Ok(msg) = rx.recv() {
                match msg {
                    WqHandler::Add(state) => {
                        wq.push(state);
                    }
                    WqHandler::Pop(tx) => match (wq.pop(), max_steps) {
                        (Some(state), _) => {
                            tx.send(WqState::Wrk(state)).unwrap();
                        }
                        (_, 1000..) => {
                            tx.send(WqState::Done).unwrap();
                        }
                        (None, _) => {
                            tx.send(WqState::Retry).unwrap();
                        }
                    },
                    WqHandler::End(state) => {
                        max_steps = std::cmp::max(state.steps, max_steps);
                    }
                }
            }
            max_steps
        });

        for _i in 0..threads.into() {
            let tx = tx.clone();
            let graph = graph.clone();
            s.spawn(move || 'outer: loop {
                let (tx2, rx2) = std::sync::mpsc::channel::<WqState>();
                tx.send(WqHandler::Pop(tx2)).unwrap();
                let mut wq = Vec::new();
                while let Ok(state) = rx2.recv() {
                    match state {
                        WqState::Wrk(state) => {
                            wq.push(state);
                        }
                        WqState::Done => {
                            break 'outer;
                        }
                        WqState::Retry => {
                            sleep(std::time::Duration::from_millis(2));
                        }
                    }
                }
                while let Some(mut state) = wq.pop() {
                    if state.position == end {
                        tx.send(WqHandler::End(state)).unwrap();
                    } else {
                        state.visited |= 1 << state.position.index();
                        for (i, nextnode) in graph.neighbors(state.position).enumerate() {
                            let edge = graph.find_edge(state.position, nextnode).unwrap();
                            let weigh = graph.edge_weight(edge).unwrap();
                            if state.visited & (1 << nextnode.index()) == 0 {
                                let state = RouteState {
                                    steps: state.steps + weigh,
                                    position: nextnode,
                                    visited: state.visited.clone(),
                                };
                                if i == 1 && state.steps < 1500 {
                                    tx.send(WqHandler::Add(state)).unwrap();
                                } else {
                                    wq.push(state);
                                }
                            }
                        }
                    }
                }
            });
        }
        drop(tx);
        max_steps.join().unwrap()
    });

    res
}

fn find_neighbor_node2(start: Pos, map: &[Vec<Terrain>], nodes: &[Pos]) -> Vec<(Pos, usize)> {
    let mut ret = vec![];
    let mut wq = VecDeque::new();
    let mut visited = HashSet::default();
    wq.push_back((start, 0));
    visited.insert(start);
    while let Some((pos, steps)) = wq.pop_front() {
        for (n, _dir) in pos.neighbors(map) {
            if !matches!(map[n.yus()][n.xus()], Terrain::SteepSlope(_) | Terrain::Path) || visited.contains(&n) {
                continue;
            } else if nodes.contains(&n) {
                ret.push((n, steps + 1));
            } else {
                visited.insert(n);
                wq.push_back((n, steps + 1));
            }
        }
    }

    ret
}
fn find_neighbor_node1(start: Pos, map: &[Vec<Terrain>], nodes: &[Pos]) -> Vec<(Pos, usize)> {
    let mut ret = vec![];
    let mut valid_next = VecDeque::new();
    let mut visited = HashSet::default();
    valid_next.push_back((start, 0));
    visited.insert(start);
    while let Some((pos, steps)) = valid_next.pop_front() {
        for (n, _dir) in pos.neighbors(&map) {
            match (nodes.contains(&n), visited.contains(&n), map[n.yus()][n.xus()]) {
                (true, _, _) if start != n => ret.push((n, steps + 1)),
                (_, true, _) => {}
                (_, false, Terrain::Path) => {
                    valid_next.push_back((n, steps + 1));
                    visited.insert(n);
                }
                (_, false, Terrain::Forest) => {}
                (_, false, Terrain::SteepSlope(dir)) => match dir {
                    Dir::Up => {
                        if pos.y() > n.y() {
                            valid_next.push_back((n, steps + 1));
                            visited.insert(n);
                        }
                    }
                    Dir::Down => {
                        if pos.y() < n.y() {
                            valid_next.push_back((n, steps + 1));
                            visited.insert(n);
                        }
                    }
                    Dir::Left => {
                        if pos.x() > n.x() {
                            valid_next.push_back((n, steps + 1));
                            visited.insert(n);
                        }
                    }
                    Dir::Right => {
                        if pos.x() < n.x() {
                            valid_next.push_back((n, steps + 1));
                            visited.insert(n);
                        }
                    }
                },
            }
        }
    }

    ret
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
