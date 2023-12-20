use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day20)]
fn parse<'a>(input: &'a str) -> HashMap<&'a str, PulseModule> {
    let mut ret = HashMap::new();
    for line in input.lines() {
        let (prefx, splits) = line.split_once(" -> ").unwrap();
        let targets: Vec<&str> = splits.split(", ").collect();
        match line.get(0..1).unwrap() {
            "b" => ret.insert("broadcaster", PulseModule::In(targets)),
            "%" => ret.insert(
                prefx.trim_start_matches('%'),
                PulseModule::FlipFlop((HashMap::new(), targets)),
            ),
            "&" => ret.insert(
                prefx.trim_start_matches('%'),
                PulseModule::Conjunction((HashMap::new(), targets)),
            ),
            _ => None,
        };
    }
    ret
}

#[aoc(day20, part1)]
fn part1<'a>(input: &'a HashMap<&str, PulseModule>) -> String {
    todo!()
}

#[aoc(day20, part2)]
fn part2<'a>(input: &'a HashMap<&str, PulseModule>) -> String {
    todo!()
}

enum PulseModule<'a> {
    In(Vec<&'a str>),
    FlipFlop((HashMap<&'a str, u32>, Vec<&'a str>)),
    Conjunction((HashMap<&'a str, u32>, Vec<&'a str>)),
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
