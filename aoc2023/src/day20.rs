use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
#[aoc_generator(day20)]
fn parse<'a>(input: &str) -> HashMap<String, PulseModule> {
    let mut ret = HashMap::new();
    for line in input.lines() {
        let line = line.trim_start();
        let (prefx, splits) = line.split_once(" -> ").unwrap();
        let targets: Vec<String> = splits.split(", ").map(|s| s.to_string()).collect();
        match line.get(0..1).unwrap() {
            "b" => ret.insert("broadcaster".to_string(), PulseModule::In(targets)),
            "%" => ret.insert(
                prefx.trim_start_matches('%').to_string(),
                PulseModule::FlipFlop((Signal::None, targets)),
            ),
            "&" => ret.insert(
                prefx.trim_start_matches('&').to_string(),
                PulseModule::Conjunction((Signal::None, HashMap::new(), targets)),
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
        if let Some(target) = ret.get_mut(&trgt) {
            match target {
                PulseModule::Conjunction((_, states, _)) => {
                    states.insert(con.clone(), Signal::None);
                }
                _ => (),
            }
        }
    }
    ret
}
fn send_puls(module: &mut PulseModule, signal: Signal, src: String) -> Vec<(String, Signal)> {
    let mut ret = vec![];
    match module {
        PulseModule::Conjunction((_curstate, states, targets)) => {
            states.insert(src, signal);
            let outb = states
                .iter()
                .fold(true, |acc, (_, state)| *state == Signal::High && acc);
            let out = if !outb { Signal::High } else { Signal::Low };
            for target in targets {
                ret.push((target.clone(), out));
            }
        }

        PulseModule::FlipFlop((curstate, targets)) => {
            let mut changed = false;
            let out = if signal == Signal::Low {
                changed = true;
                if *curstate == Signal::High {
                    Signal::Low
                } else {
                    Signal::High
                }
            } else {
                *curstate
            };
            *curstate = out;
            if changed {
                for target in targets {
                    ret.push((target.clone(), out));
                }
            }
        }
        PulseModule::In(targets) => {
            for target in targets {
                ret.push((target.clone(), Signal::Low));
            }
        }
    }
    ret
}

#[aoc(day20, part1)]
fn part1(input: &HashMap<String, PulseModule>) -> u64 {
    let mut map = input.clone();

    let mut low_sum = 0;
    let mut high_sum = 0;
    for _ in 0..1000 {
        let mut que = VecDeque::new();
        // println!("{map:?}");
        que.push_back(("broadcaster".to_string(), "button".to_string(), Signal::Low));
        while !que.is_empty() {
            let (currnm, source, signal) = que.pop_front().unwrap();
            match signal {
                Signal::High => high_sum += 1,
                Signal::Low => low_sum += 1,
                _ => (),
            }
            // println!("{source} -{signal:?}-> {currnm}");
            if let Some(curr) = map.get_mut(&currnm) {
                for (dst, sign) in send_puls(curr, signal, source).iter() {
                    que.push_back((dst.clone(), currnm.clone(), *sign));
                }
            }
        }
    }
    println!("high: {high_sum}  low: {low_sum}");
    low_sum * high_sum
}

#[aoc(day20, part2)]
fn part2(input: &HashMap<String, PulseModule>) -> u64 {
    let mut map = input.clone();
    let mut rxparent = "None".to_string();
    let mut rxparent_len = 0;
    for (s, pm) in &map {
        match pm {
            PulseModule::Conjunction((_, parent, target)) if target.contains(&"rx".to_string()) => {
                rxparent_len = parent.len();
                rxparent = s.clone();
                break;
            }
            _ => (),
        }
    }

    let mut seen: Vec<String> = vec![];
    let mut lcms: Vec<u64> = vec![];
    let mut res = 0;
    'outer: loop {
        res += 1;
        let mut que = VecDeque::new();
        que.push_back(("broadcaster".to_string(), "button".to_string(), Signal::Low));
        while !que.is_empty() {
            let (currnm, source, signal) = que.pop_front().unwrap();
            if &currnm == &rxparent && signal == Signal::High {
                if !seen.contains(&source) {
                    seen.push(source.clone());
                    lcms.push(res);
                }
                if seen.len() == rxparent_len {
                    break 'outer;
                }
            }
            if let Some(curr) = map.get_mut(&currnm) {
                for (dst, sign) in send_puls(curr, signal, source).iter() {
                    que.push_back((dst.clone(), currnm.clone(), *sign));
                }
            }
        }
    }
    lcms.into_iter().reduce(|acc, x| lcm(acc, x)).unwrap()
}

#[derive(Debug, Clone)]
enum PulseModule {
    In(Vec<String>),
    FlipFlop((Signal, Vec<String>)),
    Conjunction((Signal, HashMap<String, Signal>, Vec<String>)),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Signal {
    High,
    Low,
    None,
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = "broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a";
    const TESTINPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part1_example1() {
        let ins = parse(TESTINPUT1);
        assert_eq!(part1(&ins), 32000000);
    }
    #[test]
    fn part1_example2() {
        let ins = parse(TESTINPUT2);
        assert_eq!(part1(&ins), 11687500);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
