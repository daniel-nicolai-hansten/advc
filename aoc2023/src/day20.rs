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
                PulseModule::FlipFlop((HashMap::new(), targets)),
            ),
            "&" => ret.insert(
                prefx.trim_start_matches('%').to_string(),
                PulseModule::Conjunction((HashMap::new(), targets)),
            ),
            _ => None,
        };
    }
    ret
}

#[aoc(day20, part1)]
fn part1(input: &HashMap<String, PulseModule>) -> String {
    todo!()
}

#[aoc(day20, part2)]
fn part2(input: &HashMap<String, PulseModule>) -> String {
    todo!()
}

enum PulseModule {
    In(Vec<String>),
    FlipFlop((HashMap<String, u32>, Vec<String>)),
    Conjunction((HashMap<String, u32>, Vec<String>)),
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
