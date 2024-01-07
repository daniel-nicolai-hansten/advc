use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    character::complete::char,
    character::complete::{alpha1, digit1, one_of},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use rustc_hash::FxHashMap as HashMap;
use std::collections::VecDeque;

#[aoc_generator(day19)]
fn parse(input: &str) -> (HashMap<String, Vec<Action>>, Vec<Part>) {
    let mut workflows = HashMap::default();
    let mut parts = vec![];
    for line in input.lines() {
        if let Ok((_, (name, wf))) = workflow(line) {
            workflows.insert(name.to_string(), wf);
        } else if let Ok((_, part)) = part(line) {
            parts.push(part);
        } else {
            // println!("parse err");
        }
    }
    (workflows, parts)
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
                    // println!("accepted");
                    sum += part.x + part.m + part.a + part.s;
                    break;
                }
                "R" => {
                    // println!("rejected");
                    break;
                }
                _ => (),
            }
        }
    }
    sum
}
#[aoc(day19, part2)]
fn part2(input: &(HashMap<String, Vec<Action>>, Vec<Part>)) -> u64 {
    let (workflows, _parts) = input;
    let mut wq = VecDeque::new();
    let mut accepted = vec![];
    wq.push_back((Range::new(), "in".to_string()));
    while !wq.is_empty() {
        if let Some((mut range, trgt)) = wq.pop_front() {
            match trgt.as_str() {
                "A" => accepted.push(range),
                "R" => (),
                _ => parse_wf_for_part_2(&mut range, workflows.get(&trgt).unwrap())
                    .into_iter()
                    .filter(|(r, _)| r.valid())
                    .for_each(|nx| wq.push_back(nx)),
            }
        }
    }

    let mut sum = 0;
    for range in accepted {
        // println!("{range:?}");
        sum += range.sum_range();
    }
    sum
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
fn parse_wf_for_part_2<'a>(part: &mut Range, action: &'a [Action]) -> Vec<(Range, String)> {
    let mut ret = vec![];
    for acc in action {
        match &acc {
            &Action::Less((c, num, target)) => match c {
                'x' if part.x_l < *num => ret.push((part.x_l(*num), target.clone())),
                'm' if part.m_l < *num => ret.push((part.m_l(*num), target.clone())),
                'a' if part.a_l < *num => ret.push((part.a_l(*num), target.clone())),
                's' if part.s_l < *num => ret.push((part.s_l(*num), target.clone())),
                _ => (),
            },
            &Action::More((c, num, target)) => match c {
                'x' if part.x_h > *num => ret.push((part.x_h(*num), target.clone())),
                'm' if part.m_h > *num => ret.push((part.m_h(*num), target.clone())),
                'a' if part.a_h > *num => ret.push((part.a_h(*num), target.clone())),
                's' if part.s_h > *num => ret.push((part.s_h(*num), target.clone())),
                _ => (),
            },
            &Action::Send(target) => ret.push((part.clone(), target.clone())),
        }
    }
    ret
}

#[derive(Debug, Clone)]
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
    fn x_h(&mut self, high: u32) -> Self {
        let mut ret = self.clone();
        ret.x_l = high + 1;
        self.x_h = high;
        ret
    }
    fn m_h(&mut self, high: u32) -> Self {
        let mut ret = self.clone();
        ret.m_l = high + 1;
        self.m_h = high;
        ret
    }
    fn a_h(&mut self, high: u32) -> Self {
        let mut ret = self.clone();
        ret.a_l = high + 1;
        self.a_h = high;
        ret
    }
    fn s_h(&mut self, high: u32) -> Self {
        let mut ret = self.clone();
        ret.s_l = high + 1;
        self.s_h = high;
        ret
    }
    fn x_l(&mut self, low: u32) -> Self {
        let mut ret = self.clone();
        ret.x_h = low - 1;
        self.x_l = low;
        ret
    }
    fn m_l(&mut self, low: u32) -> Self {
        let mut ret = self.clone();
        ret.m_h = low - 1;
        self.m_l = low;
        ret
    }
    fn a_l(&mut self, low: u32) -> Self {
        let mut ret: Range = self.clone();
        ret.a_h = low - 1;
        self.a_l = low;
        ret
    }
    fn s_l(&mut self, low: u32) -> Self {
        let mut ret = self.clone();
        ret.s_h = low - 1;
        self.s_l = low;
        ret
    }
    fn new() -> Self {
        Self {
            x_h: 4000,
            x_l: 1,
            m_h: 4000,
            m_l: 1,
            a_h: 4000,
            a_l: 1,
            s_h: 4000,
            s_l: 1,
        }
    }
    fn valid(&self) -> bool {
        self.x_h > self.x_l && self.m_h > self.m_l && self.a_h > self.a_l && self.s_h > self.s_l
    }
    fn sum_range(&self) -> u64 {
        let x = 1 + self.x_h as u64 - self.x_l as u64;
        let m = 1 + self.m_h as u64 - self.m_l as u64;
        let a = 1 + self.a_h as u64 - self.a_l as u64;
        let s = 1 + self.s_h as u64 - self.s_l as u64;
        x * m * a * s
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
    // #[test]
    // fn part1_example() {
    //     assert_eq!(part1(&parse(TESTINPUT)), 19114);
    // }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 167409079868000);
    }
}
