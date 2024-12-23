use rustc_hash::FxHashSet as HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
};
use petgraph::graph::{NodeIndex, UnGraph};

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<(String, String)> {
    let (i, o) = separated_list1(newline::<&str, nom::error::Error<&str>>, separated_pair(alpha1, tag("-"), alpha1))(input).unwrap();
    assert!(i.is_empty());
    o.into_iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
}

#[aoc(day23, part1)]
fn part1(input: &[(String, String)]) -> usize {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut nodes = std::collections::HashMap::new();

    for (a, b) in input {
        let a = *nodes.entry(a.clone()).or_insert_with(|| graph.add_node(a.clone()));
        let b = *nodes.entry(b.clone()).or_insert_with(|| graph.add_node(b.clone()));
        graph.add_edge(a, b, ());
    }
    let t_nodes: Vec<(&String, &NodeIndex)> = nodes.iter().filter(|(n, _idx)| n.starts_with("t")).collect();
    let mut t_combinations = HashSet::default();
    for node in t_nodes {
        let mut neighbors = graph.neighbors(*node.1).collect::<Vec<_>>();
        neighbors.sort_by_key(|x| x.index());
        neighbors.dedup();

        for pair in neighbors.into_iter().combinations(2) {
            let mut three_list = Vec::new();
            if graph.find_edge(pair[0], pair[1]).is_none() {
                continue;
            }
            three_list.extend_from_slice(&pair);
            three_list.push(*node.1);
            three_list.sort_by_key(|x| x.index());
            t_combinations.insert(three_list);
        }
    }

    t_combinations.len()
}

#[aoc(day23, part2)]
fn part2(input: &[(String, String)]) -> String {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut nodes = std::collections::HashMap::new();

    for (a, b) in input {
        let a = *nodes.entry(a.clone()).or_insert_with(|| graph.add_node(a.clone()));
        let b = *nodes.entry(b.clone()).or_insert_with(|| graph.add_node(b.clone()));
        graph.add_edge(a, b, ());
    }
    let mut cliques = Vec::new();
    bron_kerbosch(
        &graph,
        &mut HashSet::default(),
        nodes.iter().map(|(_s, a)| *a).collect(),
        HashSet::default(),
        &mut cliques,
    );
    let p2 = cliques
        .iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .iter()
        .map(|x| nodes.iter().find(|(_s, a)| **a == *x).unwrap().0)
        .sorted()
        .join(",");
    p2
}

// algorithm BronKerbosch1(R, P, X) is
//     if P and X are both empty then
//         report R as a maximal clique
//     for each vertex v in P do
//         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}
fn bron_kerbosch<'a>(
    g: &UnGraph<String, ()>,
    r: &mut HashSet<NodeIndex>,
    mut p: HashSet<NodeIndex>,
    mut x: HashSet<NodeIndex>,
    cliques: &mut Vec<HashSet<NodeIndex>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    }
    while let Some(&n) = p.iter().next() {
        let neighbours = g.neighbors(n).collect();
        let p2 = p.intersection(&neighbours).copied().collect();
        let x2 = x.intersection(&neighbours).cloned().collect();
        r.insert(n);
        bron_kerbosch(g, r, p2, x2, cliques);
        r.remove(&n);
        p.remove(&n);
        x.insert(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), "co,de,ka,ta");
    }
}
