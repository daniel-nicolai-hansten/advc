use aoc_runner_derive::{aoc, aoc_generator};
use nom::Parser;
use nom::character::complete::{char, one_of, space0};
use nom::multi::{many1, separated_list1};
use nom::{IResult, bytes::complete::tag, sequence::delimited};

use std::collections::{HashSet, VecDeque};
use std::vec;
#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)> {
    let mut ret = vec![];
    for line in input.lines() {
        let (_rest, v) = lineparse(line).unwrap();
        ret.push(v);
    }
    ret
}
fn lineparse(input: &str) -> IResult<&str, (Vec<bool>, Vec<Vec<u64>>, Vec<u64>)> {
    let (input, bools) = delimited(tag("["), many1(one_of(".#").map(|c| c == '#')), tag("]")).parse(input)?;
    let (input, _) = space0(input)?;
    let (input, switches) = separated_list1(space0, delimited(tag("("), separated_list1(char(','), nom::character::complete::u64), tag(")"))).parse(input)?;
    let (input, _) = space0(input)?;
    let (input, jolts) = delimited(tag("{"), separated_list1(char(','), nom::character::complete::u64), tag("}")).parse(input)?;
    Ok((input, (bools, switches, jolts)))
}

#[aoc(day10, part1)]
fn part1(input: &[(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)]) -> u64 {
    let mut total = 0;
    for (lights, switches, _jolts) in input {
        let current_light = vec![false; lights.len()];
        let mut visited: HashSet<Vec<bool>> = HashSet::default();
        let mut wq = VecDeque::new();
        wq.push_back((current_light.clone(), 0));
        while let Some((state, step)) = wq.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());
            if state == *lights {
                total += step;
                break;
            }
            for switch in switches {
                let mut new_state = state.clone();
                for &idx in switch {
                    let idx = idx as usize;
                    new_state[idx] = !new_state[idx];
                }
                wq.push_back((new_state, step + 1));
            }
        }
    }
    total
}

#[aoc(day10, part2)]
fn part2(input: &[(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)]) -> u64 {
    use good_lp::*;
    let mut sum = 0;
    for (_lights, switches, jolts) in input {
        let mut vars = variables!();
        let press_vars = (0..switches.len()).map(|_| vars.add(variable().min(0).integer())).collect::<Vec<_>>();

        let mut problem = highs(vars.minimise(press_vars.iter().sum::<Expression>()));
        let mut exprs = vec![0.into_expression(); jolts.len()];
        for (i, x) in switches.iter().enumerate() {
            for &x in x {
                exprs[x as usize] += press_vars[i];
            }
        }
        for (e, &j) in exprs.into_iter().zip(jolts) {
            problem.add_constraint(e.eq(j as f64));
        }
        let sol = problem.solve().unwrap();
        sum += press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 33);
    }
}
