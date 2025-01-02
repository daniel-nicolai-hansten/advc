use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::{graph, visit::EdgeRef, Graph, Undirected};
#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let splits = line.split_once("-").unwrap();
            (splits.0.to_string(), splits.1.to_string())
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[(String, String)]) -> usize {
    let mut graph: Graph<&str, (), Undirected> = graph::Graph::new_undirected();
    let mut nodes: HashMap<&str, petgraph::prelude::NodeIndex> = HashMap::new();
    let mut ret = 0;
    for (from, to) in input {
        let from = *nodes.entry(from).or_insert_with(|| graph.add_node(from));
        let to = *nodes.entry(to).or_insert_with(|| graph.add_node(to));
        graph.add_edge(from, to, ());
    }
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((nodes["start"], vec![nodes["start"]]));
    while let Some((node, path)) = queue.pop_front() {
        if node == nodes["end"] {
            ret += 1;
            continue;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            let is_bigcave = graph.node_weight(next).unwrap().chars().next().unwrap().is_ascii_uppercase();
            if !is_bigcave && path.contains(&next) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(next);
            queue.push_back((next, new_path));
        }
    }

    ret
}

#[aoc(day12, part2)]
fn part2(input: &[(String, String)]) -> usize {
    let mut graph: Graph<&str, (), Undirected> = graph::Graph::new_undirected();
    let mut nodes: HashMap<&str, petgraph::prelude::NodeIndex> = HashMap::new();
    let mut ret = 0;
    for (from, to) in input {
        let from = *nodes.entry(from).or_insert_with(|| graph.add_node(from));
        let to = *nodes.entry(to).or_insert_with(|| graph.add_node(to));
        graph.add_edge(from, to, ());
    }
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((nodes["start"], vec![nodes["start"]], false));
    while let Some((node, path, extra)) = queue.pop_front() {
        if node == nodes["end"] {
            ret += 1;
            continue;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            let mut extra = extra;
            let is_bigcave = graph.node_weight(next).unwrap().chars().next().unwrap().is_ascii_uppercase();
            if !is_bigcave && path.contains(&next) {
                if extra || graph.node_weight(next).unwrap() == &"start" {
                    continue;
                } else {
                    extra = true;
                }
            }
            let mut new_path = path.clone();
            new_path.push(next);
            queue.push_back((next, new_path, extra));
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    const TESTINPUT2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 10);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT2)), 103);
    }
}
