use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::prelude::*;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use std::{collections::HashMap, error::Error};

#[aoc_generator(day25)]
fn parse(input: &str) -> (Graph<String, u32, Undirected>, usize) {
    let mut graph = UnGraph::default();
    let mut nodemap = HashMap::new();
    let mut ret_connections = vec![];
    for line in input.lines() {
        let (name, rest) = line.trim().split_once(':').unwrap();
        let connections: Vec<_> = rest.split_ascii_whitespace().collect();
        if !nodemap.contains_key(name) {
            let idx = graph.add_node(name.to_string());
            nodemap.insert(name.to_string(), idx);
        }
        for con in connections {
            if !nodemap.contains_key(con) {
                let idx = graph.add_node(con.to_string());
                nodemap.insert(con.to_string(), idx);
            }
            ret_connections.push((con, name));
        }
    }
    for (conn, name) in ret_connections {
        let from_idx = nodemap.get(name).unwrap();
        let to_idx = nodemap.get(conn).unwrap();
        graph.add_edge(*from_idx, *to_idx, 1);
    }
    (graph, nodemap.len())
}

#[aoc(day25, part1)]
fn part1(input: &(Graph<String, u32, Undirected>, usize)) -> usize {
    let (graph, totalnodes) = input;
    let min = stoer_wagner_min_cut(graph, |_| Ok::<i32, Box<dyn Error>>(1));
    let (_cut_size, part_nodes) = min.unwrap().unwrap();
    let part1 = part_nodes.len();
    let part2 = totalnodes - part1;
    part1 * part2
}

#[aoc(day25, part2)]
fn part2(_input: &(Graph<String, u32, Undirected>, usize)) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 54);
    }
}
