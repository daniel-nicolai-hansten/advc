use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(oasis_historys: &[Vec<i64>]) -> i64 {
    oasis_historys
        .iter()
        .fold(0, |acc, history| acc + predict(history, &Dir::Fwd))
}

#[aoc(day9, part2)]
fn part2(oasis_historys: &[Vec<i64>]) -> i64 {
    oasis_historys
        .iter()
        .fold(0, |acc, history| acc + predict(history, &Dir::Rev))
}

pub fn predict(values: &[i64], dir: &Dir) -> i64 {
    let diffs: Vec<i64> = values.windows(2).map(|v| v[1] - v[0]).collect();
    let allzeros = !values.iter().fold(false, |acc, x| acc || *x != 0);
    match (allzeros, dir) {
        (true, _) => 0,
        (false, Dir::Fwd) => values.last().unwrap() + predict(&diffs, dir),
        (false, Dir::Rev) => values.first().unwrap() - predict(&diffs, dir),
    }
}
pub enum Dir {
    Fwd,
    Rev,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 2);
    }
}
