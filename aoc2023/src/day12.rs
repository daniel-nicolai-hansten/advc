use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
#[aoc_generator(day12, part1)]
fn parse(input: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
    let mut ret = vec![];
    let gearsplit = |c| match c {
        '#' => Spring::Broken,
        '.' => Spring::Working,
        '?' => Spring::Unknown,
        _ => Spring::Err,
    };
    for line in input.lines() {
        let splits: Vec<&str> = line.trim_start().split_ascii_whitespace().collect();
        let gearstates: Vec<Spring> = splits[0].chars().map(gearsplit).collect();
        let nums: Vec<usize> = splits[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        ret.push((gearstates, nums));
    }
    ret
}
#[aoc_generator(day12, part2)]
fn parse2(input: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
    let mut ret = vec![];
    let gearsplit = |c| match c {
        '#' => Spring::Broken,
        '.' => Spring::Working,
        '?' => Spring::Unknown,
        _ => Spring::Err,
    };
    for line in input.lines() {
        let splits: Vec<&str> = line.trim_start().split_ascii_whitespace().collect();
        let gearstates: Vec<Spring> = splits[0].chars().map(gearsplit).collect();
        let nums: Vec<usize> = splits[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let mut gearstates_p2: Vec<Spring> = vec![];
        let mut nums_p2: Vec<usize> = vec![];
        for i in 0..5 {
            let mut new_gearstates = gearstates.clone();
            if i < 4 {
                new_gearstates.push(Spring::Unknown);
            }
            gearstates_p2.append(&mut new_gearstates);
            nums_p2.append(&mut nums.clone());
        }
        ret.push((gearstates_p2, nums_p2));
    }
    ret
}

#[aoc(day12, part1)]
fn part1(springs: &[(Vec<Spring>, Vec<usize>)]) -> usize {
    springs
        .par_iter()
        .map(|(spring, nums)| count_variants(&spring, nums))
        .sum()
}

#[aoc(day12, part2)]
fn part2(springs: &[(Vec<Spring>, Vec<usize>)]) -> usize {
    springs
        .par_iter()
        .map(|(spring, nums)| count_variants(&spring, nums))
        .sum()
}

fn count_variants(springs: &[Spring], counts: &[usize]) -> usize {
    let tot_springs = springs.len();
    let tot_groups = counts.len();
    let mut dp = vec![vec![vec![0; tot_springs + 1]; tot_groups + 1]; tot_springs + 1];
    dp[tot_springs][tot_groups][0] = 1;
    dp[tot_springs][tot_groups - 1][counts[tot_groups - 1]] = 1;
    use Spring as S;
    for springnum in (0..tot_springs).rev() {
        for (group, &max_count) in counts.iter().enumerate() {
            for count in 0..=max_count {
                for c in [S::Working, S::Broken] {
                    match (springs[springnum] == c || springs[springnum] == S::Unknown, c) {
                        (true, S::Working) if count == 0 => {
                            dp[springnum][group][count] += dp[springnum + 1][group][0];
                        }
                        (true, S::Working) if group < tot_groups && counts[group] == count => {
                            dp[springnum][group][count] += dp[springnum + 1][group + 1][0];
                        }
                        (true, S::Broken) => {
                            dp[springnum][group][count] += dp[springnum + 1][group][count + 1];
                        }
                        _ => (),
                    }
                }
            }
        }
        if matches!(springs[springnum], Spring::Working | Spring::Unknown) {
            dp[springnum][tot_groups][0] += dp[springnum + 1][tot_groups][0];
        }
    }
    dp[0][0][0]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Broken,
    Working,
    Unknown,
    Err,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
//     }
// }
