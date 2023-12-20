use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day20)]
fn parse<'a>(input: &str) -> HashMap<String, PulseModule> {
    let mut ret = HashMap::new();
    for line in input.lines() {
        let (prefx, splits) = line.split_once(" -> ").unwrap();
        let targets: Vec<String> = splits.split(", ").map(|s| s.to_string()).collect();
        match line.get(0..1).unwrap() {
            "b" => ret.insert("broadcaster".to_string(), PulseModule::In(targets)),
            "%" => ret.insert(
                prefx.trim_start_matches('%').to_string(),
                PulseModule::FlipFlop((false, targets)),
            ),
            "&" => ret.insert(
                prefx.trim_start_matches('&').to_string(),
                PulseModule::Conjunction((false, HashMap::new(), targets)),
            ),
            _ => None,
        };
    }
    let mut connections = vec![];
    for (con, module) in &ret {
        match module {
            PulseModule::Conjunction((_, _states, targets)) => {
                for trgt in targets {
                    connections.push((con.clone(), trgt.clone()));
                }
            }
            PulseModule::FlipFlop((_state, targets)) => {
                for trgt in targets {
                    connections.push((con.clone(), trgt.clone()));
                }
            }
            _ => (),
        }
    }

    for (con, trgt) in connections {
        let target = ret.get_mut(&trgt).unwrap();
        match target {
            PulseModule::Conjunction((_, states, _)) => {
                states.insert(con.clone(), false);
            }
            _ => (),
        }
    }
    ret
}
fn send_puls(module: &mut PulseModule, signal: bool, src: String) -> Vec<(String, bool)> {
    let mut ret = vec![];
    match module {
        PulseModule::Conjunction((curstate, states, targets)) => {
            states.insert(src, signal);
            let out = states.iter().fold(true, |acc, (_, state)| *state && acc);
            for target in targets {
                ret.push((target.clone(), out));
            }
        }
        PulseModule::FlipFlop((curstate, targets)) => {
            let out = if !signal { !*curstate } else { *curstate };
            *curstate = out;
            for target in targets {
                ret.push((target.clone(), out));
            }
        }
        PulseModule::In(targets) => {
            for target in targets {
                ret.push((target.clone(), false));
            }
        }
    }
    ret
}

#[aoc(day20, part1)]
fn part1(input: &HashMap<String, PulseModule>) -> String {
    // for inm in input {
    //     println!("{inm:?}");
    // }
    for _i in 0..10 {
        let mut stack = vec![];

        stack.push(("broadcaster".to_string(), false));
        let mut currnm = "broadcaster".to_string();
        loop {
            let (currnm, signal) = stack.pop().unwrap();
            let curr = input.get(&currnm).unwrap();
            for res in send_puls(curr, signal, src) {}
            if stack.is_empty() {
                break;
            }
        }
    }
    "ok".to_string()
}

#[aoc(day20, part2)]
fn part2(input: &HashMap<String, PulseModule>) -> String {
    todo!()
}
#[derive(Debug)]
enum PulseModule {
    In(Vec<String>),
    FlipFlop((bool, Vec<String>)),
    Conjunction((bool, HashMap<String, bool>, Vec<String>)),
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    #[test]
    fn part1_example() {
        let ins = parse(TESTINPUT);
        assert_eq!(part1(&ins), "<RESULT>");
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
