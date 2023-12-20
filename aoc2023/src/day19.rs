use std::cmp::min;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::take_till1;
use nom::character::complete::{alpha1, digit1, one_of};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::char,
    sequence::delimited,
    IResult,
};
#[aoc_generator(day19)]
fn parse(input: &str) -> (HashMap<String, Vec<Action>>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = vec![];
    for line in input.lines() {
        if let Ok((_, (name, wf))) = workflow(line) {
            workflows.insert(name.to_string(), wf);
        } else if let Ok((_, part)) = part(line) {
            parts.push(part);
        } else {
            println!("parse err");
        }
    }
    (workflows, parts)
}
fn part(input: &str) -> IResult<&str, Part> {
    let (_, nom) = delimited(char('{'), is_not("}"), char('}'))(input)?;
    let (nom, partraw) = separated_list1(tag(","), part_val)(nom)?;
    let mut part = Part { x: 0, m: 0, a: 0, s: 0 };
    for (c, val) in partraw {
        match c {
            'x' => part.x = val,
            'm' => part.m = val,
            'a' => part.a = val,
            's' => part.s = val,
            _ => (),
        }
    }
    Ok((nom, part))
}
fn part_val(input: &str) -> IResult<&str, (char, u32)> {
    let (nom, s) = one_of("xmas")(input)?;
    let (nom, val) = preceded(tag("="), digit1)(nom)?;
    Ok((nom, (s, val.parse().unwrap())))
}
fn workflow(input: &str) -> IResult<&str, (&str, Vec<Action>)> {
    let (nom, name) = take_till1(|c| c == '{')(input)?;
    let (_, nom) = delimited(char('{'), is_not("}"), char('}'))(nom)?;
    let (nom, action) = separated_list1(tag(","), alt((workflow_action_ml, workflow_action_sar)))(nom)?;
    // println!("{action:?}");
    Ok((nom, (name, action)))
}
fn workflow_action_ml(input: &str) -> IResult<&str, Action> {
    let (nom, val) = one_of("xmas")(input)?;
    let (nom, c) = one_of("<>")(nom)?;
    let (nom, num) = digit1(nom)?;
    let (nom, target) = preceded(tag(":"), alpha1)(nom)?;
    let ret = match c {
        '>' => Action::More((val, num.parse::<u32>().unwrap(), target.to_string())),
        '<' => Action::Less((val, num.parse::<u32>().unwrap(), target.to_string())),
        _ => unreachable!(),
    };
    Ok((nom, ret))
}
fn workflow_action_sar(input: &str) -> IResult<&str, Action> {
    let (nom, val) = alpha1(input)?;
    let ret = Action::Send(val.to_string());
    Ok((nom, ret))
}
#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
#[derive(Debug)]
enum Action {
    Less((char, u32, String)),
    More((char, u32, String)),
    Send(String),
}

#[aoc(day19, part1)]
fn part1(input: &(HashMap<String, Vec<Action>>, Vec<Part>)) -> u32 {
    let (workflows, parts) = input;
    let mut sum = 0;
    for part in parts {
        // println!("{part:?}");
        let mut trgt = "in";
        for _ in 0..500 {
            trgt = parse_wf_for_part(part, workflows.get(trgt).unwrap());
            // println!("{trgt}");
            match trgt {
                "A" => {
                    println!("accepted");
                    sum += part.x + part.m + part.a + part.s;
                    break;
                }
                "R" => {
                    println!("rejected");
                    break;
                }
                _ => (),
            }
        }
    }
    sum
}
fn parse_wf_for_part<'a>(part: &Part, action: &'a [Action]) -> &'a str {
    for acc in action {
        match &acc {
            &Action::Less((c, num, target)) => match c {
                'x' if part.x < *num => {
                    return &target;
                }
                'm' if part.m < *num => {
                    return &target;
                }
                'a' if part.a < *num => {
                    return &target;
                }
                's' if part.s < *num => {
                    return &target;
                }
                _ => (),
            },
            &Action::More((c, num, target)) => match c {
                'x' if part.x > *num => {
                    return &target;
                }
                'm' if part.m > *num => {
                    return &target;
                }
                'a' if part.a > *num => {
                    return &target;
                }
                's' if part.s > *num => {
                    return &target;
                }
                _ => (),
            },
            &Action::Send(target) => return &target,
        }
    }
    ""
}
fn parse_wf_for_part_2<'a>(part: &Range, action: &'a [Action]) -> &'a str {
    let ret = vec![];
    for acc in action {
        match &acc {
            &Action::Less((c, num, target)) => match c {
                'x' if part.x_l < *num => {

                    return &target;
                }
                'm' if part.m_l < *num => {
                    return &target;
                }
                'a' if part.a_l < *num => {
                    return &target;
                }
                's' if part.s_l < *num => {
                    return &target;
                }
                _ => (),
            },
            &Action::More((c, num, target)) => match c {
                'x' if part.x_h > *num => {
                    return &target;
                }
                'm' if part.m_h > *num => {
                    return &target;
                }
                'a' if part.a_h > *num => {
                    return &target;
                }
                's' if part.s_h > *num => {
                    return &target;
                }
                _ => (),
            },
            &Action::Send(target) => return &target,
        }
    }
    ""
}

#[aoc(day19, part2)]
fn part2(input: &(HashMap<String, Vec<Action>>, Vec<Part>)) -> String {
    todo!()
}
struct Range {
    x_h: u32,
    x_l: u32,
    m_h: u32,
    m_l: u32,
    a_h: u32,
    a_l: u32,
    s_h: u32,
    s_l: u32,
} 
impl Range {
    fn x_h(&self, high: u32) -> Self {
        let mut ret = self.clone();
        ret.x_h = min(self.x_h, high);
        *ret
    }
    fn m_h(&self, high: u32) -> Self {
        let mut ret = self.clone();
        ret.x_h = min(self.x_h, high);
        *ret
    }
    fn a_h(&self, high: u32) -> Self {
        let mut ret = self.clone();
        ret.x_h = min(self.x_h, high);
        *ret
    }
    fn s_h(&self, high: u32) -> Self {
        let mut ret = self.clone();
        ret.x_h = min(self.x_h, high);
        *ret
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 19114);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
