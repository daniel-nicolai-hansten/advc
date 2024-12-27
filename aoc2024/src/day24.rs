use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use log::debug;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, alphanumeric1, newline, space0},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

use petgraph::{
    dot::Dot,
    visit::{EdgeRef, NodeRef},
    Graph,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    debug!("{:?}  {:b}", res, tot);
    tot
}

#[aoc(day24, part2)]
fn part2(input: &(Vec<(String, u8)>, Vec<((String, Opr, String), String)>)) -> String {
    let mut graph = Graph::new();
    let mut nodes: HashMap<&str, petgraph::prelude::NodeIndex> = HashMap::new();
    let (wires, ops) = input;
    for wire in wires {
        if let Some(_) = nodes.insert(&wire.0, graph.add_node(wire.0.as_str())) {
            continue;
        }
    }
    for ((a, op, b), c) in ops {
        let a = match nodes.get(a.as_str()) {
            Some(a) => *a,
            None => {
                let idx = graph.add_node(a.as_str());
                nodes.insert(a, idx);
                idx
            }
        };
        let b = match nodes.get(b.as_str()) {
            Some(b) => *b,
            None => {
                let idx = graph.add_node(b.as_str());
                nodes.insert(b, idx);
                idx
            }
        };
        let c = match nodes.get(c.as_str()) {
            Some(c) => *c,
            None => {
                let idx = graph.add_node(c.as_str());
                nodes.insert(c, idx);
                idx
            }
        };
        graph.add_edge(a, c, *op);
        graph.add_edge(b, c, *op);
    }
    let dot = format!("{:?}", Dot::with_config(&graph, &[]));
    std::fs::write("graph.dot", dot).unwrap();
    match verify_calc(graph, nodes) {
        Ok(res) => {
            let mut res = res;
            res.sort();
            res.join(",")
        }
        _ => "Error!".to_string(),
    }
}
fn verify_calc(graph: Graph<&str, Opr>, idxmap: HashMap<&str, petgraph::prelude::NodeIndex>) -> anyhow::Result<Vec<String>> {
    let mut carry = None;
    let mut errorlist = Vec::new();
    for n in 0..45 {
        let xidx = idxmap.get(format!("x{n:02}").as_str()).unwrap();
        let yidx = idxmap.get(format!("y{n:02}").as_str()).unwrap();

        let child = |n1, n2, opr| {
            let n1 = n1?;
            let n2 = n2?;
            let xor1 = graph
                .edges(n1)
                .find_map(|n| (*n.weight() == opr).then(|| graph.edge_endpoints(n.id())))
                .flatten()?;
            let xor2 = graph
                .edges(n2)
                .find_map(|n| (*n.weight() == opr).then(|| graph.edge_endpoints(n.id())))
                .flatten()?;
            match (xor1, xor2) {
                ((_, xid), (_, yid)) if xid == yid => Some(xid),
                _ => None,
            }
        };

        if carry.is_none() {
            carry = child(Some(*xidx), Some(*yidx), Opr::AND);
            let z_out = child(Some(*xidx), Some(*yidx), Opr::XOR);
            if z_out.is_none() || graph.node_weight(z_out.unwrap().id()).unwrap() != &format!("z{n:02}").as_str() {
                errorlist.push(format!("x{n:02}"));
                errorlist.push(format!("y{n:02}"));
            }
            if carry.is_none() {
                errorlist.push(format!("x{n:02}"));
                errorlist.push(format!("y{n:02}"));
                return Ok(errorlist);
            }
        } else {
            let mut xor1 = child(Some(*xidx), Some(*yidx), Opr::XOR);
            let mut and1 = child(Some(*xidx), Some(*yidx), Opr::AND);
            let mut xor2 = match child(xor1, carry, Opr::XOR) {
                Some(res) => Some(res),
                None if child(and1, carry, Opr::XOR).is_some() => {
                    errorlist.push(graph.node_weight(xor1.unwrap()).unwrap().to_string());
                    errorlist.push(graph.node_weight(and1.unwrap()).unwrap().to_string());
                    let xor1_tmp = xor1;
                    xor1 = and1;
                    and1 = xor1_tmp;
                    child(xor1, carry, Opr::XOR)
                }
                None => None,
            };

            let mut and2 = child(xor1, carry, Opr::AND);
            carry = child(and2, and1, Opr::OR);
            let names = ["xor1", "and1", "xor2", "and2", "carry"];
            'chkloop: for _ in 0..5 {
                for (i, node) in [xor1, and1, xor2, and2, carry].iter().enumerate() {
                    if let Some(nde) = node {
                        match (names[i], graph.node_weight(*nde).unwrap() == &format!("z{n:02}").as_str()) {
                            ("xor2", false) => {
                                errorlist.push(graph.node_weight(*nde).unwrap().to_string());
                            }
                            ("and2", true) => {
                                errorlist.push(graph.node_weight(*nde).unwrap().to_string());
                                let xor2_tmp = xor2;
                                xor2 = and2;
                                and2 = xor2_tmp;
                                carry = child(and2, and1, Opr::OR);
                                debug!("swapped and2 and z{n:02}");
                            }
                            ("carry", true) => {
                                errorlist.push(graph.node_weight(*nde).unwrap().to_string());
                                let xor2_tmp = xor2;
                                xor2 = carry;
                                carry = xor2_tmp;
                                debug!("swapped carry and z{n:02}");
                            }
                            ("and1", true) => {
                                errorlist.push(graph.node_weight(*nde).unwrap().to_string());
                                let and1_tmp = and1;
                                and1 = xor2;
                                xor2 = and1_tmp;
                                carry = child(and2, and1, Opr::OR);
                                debug!("swapped and1 and z{n:02}");
                            }
                            _ => (),
                        }
                        debug!("{}: found {} at: {}", n, names[i], graph.node_weight(*nde).unwrap());
                    } else {
                        debug!("{}: missing {}", n, names[i]);
                        continue 'chkloop;
                    }
                }
                break;
            }
        }
    }

    Ok(errorlist)
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
        assert_eq!(part2(&parse(TESTINPUT)), "");
    }
}
