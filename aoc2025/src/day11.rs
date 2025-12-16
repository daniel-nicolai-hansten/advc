use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{IResult, Parser, bytes::complete::tag, multi::separated_list1};
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph};
use rustc_hash::FxHashMap as HashMap;
use std::fs::File;
use std::io::Write;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<(String, Vec<String>)> {
    let mut result = Vec::new();
    for line in input.lines() {
        if let Ok((_, parsed)) = lineparse(line) {
            result.push(parsed);
        }
    }
    result
}
fn lineparse(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, key) = nom::character::complete::alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, values) = separated_list1(nom::character::complete::space1, nom::character::complete::alpha1).parse(input)?;
    Ok((input, (key.to_string(), values.into_iter().map(|s| s.to_string()).collect())))
}

fn parse_graph(input: &[(String, Vec<String>)]) -> (Graph<&str, (), Directed>, HashMap<&str, NodeIndex>) {
    let mut graph = Graph::<&str, (), Directed>::new();
    let mut nodes = HashMap::default();
    for (source, targets) in input {
        let source_idx = *nodes.entry(source.as_str()).or_insert_with(|| graph.add_node(source.as_str()));
        for target in targets {
            let target_idx = *nodes.entry(target.as_str()).or_insert_with(|| graph.add_node(target.as_str()));
            graph.add_edge(source_idx, target_idx, ());
        }
    }
    (graph, nodes)
}

#[allow(dead_code)]
fn write_dot_file(graph: &Graph<&str, (), Directed>, filename: &str) {
    let dot_output = format!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]));
    if let Ok(mut file) = File::create(filename) {
        let _ = file.write_all(dot_output.as_bytes());
        println!("Graph written to {}", filename);
    }
}

#[aoc(day11, part1)]
fn part1(input: &[(String, Vec<String>)]) -> u64 {
    let (graph, nodes) = parse_graph(input);
    let start = nodes.get("you").unwrap();
    let end = nodes.get("out").unwrap();
    dfs(&graph, *start, *end, &mut HashMap::default())
}

#[aoc(day11, part2)]
fn part2(input: &[(String, Vec<String>)]) -> u64 {
    let (graph, nodes) = parse_graph(input);
    // write_dot_file(&graph, "graph_part2.dot");

    ["svr", "fft", "dac", "out"]
        .iter()
        .tuple_windows()
        .map(|(start, end)| dfs(&graph, *nodes.get(start).unwrap(), *nodes.get(end).unwrap(), &mut HashMap::default()))
        .product()

    // let pathcount_fft = dfs(&graph, *nodes.get("svr").unwrap(), *nodes.get("fft").unwrap(), &mut HashMap::default());
    // let pathcount_dac = dfs(&graph, *nodes.get("fft").unwrap(), *nodes.get("dac").unwrap(), &mut HashMap::default());
    // let pathcount_out = dfs(&graph, *nodes.get("dac").unwrap(), *nodes.get("out").unwrap(), &mut HashMap::default());
    // pathcount_fft * pathcount_dac * pathcount_out
}

fn dfs(graph: &Graph<&str, (), Directed>, start: NodeIndex, end: NodeIndex, visited: &mut HashMap<NodeIndex, u64>) -> u64 {
    graph
        .neighbors(start)
        .map(|neighbor| match neighbor == end {
            true => 1,
            false if !visited.contains_key(&neighbor) => {
                let subpaths = dfs(graph, neighbor, end, visited);
                visited.insert(neighbor, subpaths);
                subpaths
            }
            false => *visited.get(&neighbor).unwrap_or(&0),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
    const TESTINPUT2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT2)), 2);
    }
}
