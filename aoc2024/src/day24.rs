use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, alphanumeric1, newline, space0},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use petgraph::{
    dot::{Config, Dot}, graph, Graph
};
#[aoc_generator(day24)]
fn parse(input: &str) -> (Vec<(String, u8)>, Vec<((String, Opr, String), String)>) {
    prse(input).unwrap().1
}
fn prse(input: &str) -> IResult<&str, (Vec<(String, u8)>, Vec<((String, Opr, String), String)>)> {
    let (i, o) = separated_list1(newline, separated_pair(alphanumeric1.map(|g: &str| g.to_string()), tag(": "), complete::u8))(input)?;
    let (i, _) = many1(newline)(i)?;
    let (i, o2) = separated_list1(
        newline,
        separated_pair(
            tuple((
                alphanumeric1.map(|g: &str| g.to_string()),
                space0,
                alpha1.map(|opr| match opr {
                    "AND" => Opr::AND,
                    "OR" => Opr::OR,
                    "XOR" => Opr::XOR,
                    _ => unreachable!(),
                }),
                space0,
                alphanumeric1.map(|g: &str| g.to_string()),
            ))
            .map(|(a, _, b, _, c)| ((a, b, c))),
            tag(" -> "),
            alphanumeric1.map(|g: &str| g.to_string()),
        ),
    )(i)?;
    Ok((i, (o, o2)))
}
enum Opr {
    AND,
    OR,
    XOR,
}
#[aoc(day24, part1)]
fn part1(input: &(Vec<(String, u8)>, Vec<((String, Opr, String), String)>)) -> u64 {
    let (wires, ops) = input;
    let mut wires = wires.iter().map(|(a, b)| (a.as_str(), *b)).collect::<std::collections::HashMap<&str, u8>>();
    loop {
        let mut missing = false;
        for ((a, op, b), c) in ops {
            match (wires.get(a.as_str()), wires.get(b.as_str()), op) {
                (Some(siga), Some(sigb), Opr::AND) => {
                    wires.insert(c.as_str(), siga & sigb);
                }
                (Some(siga), Some(sigb), Opr::OR) => {
                    wires.insert(c.as_str(), siga | sigb);
                }
                (Some(siga), Some(sigb), Opr::XOR) => {
                    wires.insert(c.as_str(), siga ^ sigb);
                }
                _ => {
                    missing = true;
                }
            }
        }
        if !missing {
            break;
        }
    }
    let res: Vec<(&str, u8)> = wires
        .iter()
        .filter_map(|(&wires, &value)| wires.starts_with("z").then_some((wires, value)))
        .sorted()
        .rev()
        .collect();
    let tot = res.iter().fold(0_u64, |acc, (_wires, value)| acc << 1 | *value as u64);

    println!("{:?}  {:b}", res, tot);
    tot
}
fn run_calculator(x: u64, y: u64, ops: &Vec<((String, Opr, String), String)>) -> u64 {
    let mut wires = HashMap::new();
    for i in 0..u64::BITS {
        let xname = format!("x{:02}", i);
        let yname = format!("y{:02}", i);
        wires.insert(xname, ((x >> i) & 1) as u8);
        wires.insert(yname, ((y >> i) & 1) as u8);
    }
    loop {
        let mut missing = false;
        for ((a, op, b), c) in ops {
            match (wires.get(a.as_str()), wires.get(b.as_str()), op) {
                (Some(siga), Some(sigb), Opr::AND) => {
                    wires.insert(c.clone(), siga & sigb);
                }
                (Some(siga), Some(sigb), Opr::OR) => {
                    wires.insert(c.clone(), siga | sigb);
                }
                (Some(siga), Some(sigb), Opr::XOR) => {
                    wires.insert(c.clone(), siga ^ sigb);
                }
                _ => {
                    missing = true;
                }
            }
        }
        if !missing {
            break;
        }
    }
    let res: Vec<(String, u8)> = wires
        .iter()
        .filter_map(|(wires, &value)| wires.starts_with("z").then_some((wires.clone(), value)))
        .sorted()
        .rev()
        .collect();
    res.iter().fold(0_u64, |acc, (_wires, value)| acc << 1 | *value as u64)
}

#[aoc(day24, part2)]
fn part2(input: &(Vec<(String, u8)>, Vec<((String, Opr, String), String)>)) -> String {
    let mut graph = Graph::new_undirected();
    let mut nodes: HashMap<&str, petgraph::prelude::NodeIndex> = HashMap::new();
    let (wires, ops) = input;
    for wire in wires {
        if let Some(_) = nodes.insert(&wire.0, graph.add_node(&wire.0)) {
            continue;
        }
    }
    for ((a, op, b), c) in ops {
        let a = match nodes.get(a.as_str()) {
            Some(a) => *a,
            None => {
                let idx = graph.add_node(a);
                nodes.insert(a, idx);
                idx
            }
        };
        let b = match nodes.get(b.as_str()) {
            Some(b) => *b,
            None => {
                let idx = graph.add_node(b);
                nodes.insert(b, idx);
                idx
            }
        };
        let c = match nodes.get(c.as_str()) {
            Some(c) => *c,
            None => {
                let idx = graph.add_node(c);
                nodes.insert(c, idx);
                idx
            }
        };
        match op {
            Opr::AND => {
                graph.add_edge(a, c, "AND");
                graph.add_edge(b, c, "AND");
            }
            Opr::OR => {
                graph.add_edge(a, c, "OR");
                graph.add_edge(b, c, "OR");
            }
            Opr::XOR => {
                graph.add_edge(a, c, "XOR");
                graph.add_edge(b, c, "XOR");
            }
        }
    }
    let res = run_calculator(u64::MAX, 0, ops);
    println!("{:b}", res);
    let res = run_calculator(u64::MAX/2, u64::MAX/2 + 1, ops);
    println!("{:b}", res);
    for (&zeds, idx) in nodes.iter().filter_map(|(zeds, idx)| zeds.starts_with("z").then_some((zeds, *idx))) {
        for edge in  graph.edges(idx) {
            if *edge.weight() == "XOR" {
                continue;
            }
            println!("{:?} {:?}", zeds, edge);
        }

    }
    // output to dot file
    let dot = format!("{:?}", Dot::with_config(&graph, &[Config::GraphContentOnly]));
    std::fs::write("graph.dot", dot).unwrap();
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 2024);
    }

    #[test]
    fn part2_example() {
        // assert_eq!(part2(&parse(TESTINPUT)), "<RESULT>");
    }
}
