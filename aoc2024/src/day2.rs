use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day2)]
fn parse<'a>(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| s.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect()
}
#[derive(Clone, Copy, Debug)]
enum State {
    None,
    Asc,
    Dsc,
}
#[aoc(day2, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    let mut cnt = 0;
    for ln in input {
        if safethy(ln) {
            cnt += 1;
        }
    }
    cnt
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    let mut cnt = 0;
    for ln in input {
        if safethy(ln) {
            cnt += 1;
        } else {
            'inner: for n in 0..ln.len() {
                let damp_list: Vec<i32> = ln.iter().enumerate().filter_map(|(i, e)| (i != n).then(|| *e)).collect();
                if safethy(&damp_list) {
                    cnt += 1;
                    break 'inner;
                }
            }
        }
    }
    cnt
}

fn safethy(input: &[i32]) -> bool {
    let mut state = State::None;
    for (n1, n2) in input.iter().tuple_windows() {
        let diff = n1.abs_diff(*n2);
        match (diff, state, n1 < n2) {
            (1..=3, State::None, false) => state = State::Dsc,
            (1..=3, State::None, true) => state = State::Asc,
            (1..=3, State::Dsc, false) => (),
            (1..=3, State::Asc, true) => (),
            _ => return false,
        };
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT1)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT1)), 4);
    }
}
