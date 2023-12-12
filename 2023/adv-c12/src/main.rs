use rayon::prelude::*;
use std::sync::mpsc;
fn main() {
    let input = include_str!("../input.txt");
    let springs = parse_input(input);
    let (tx, rx) = mpsc::channel();
    springs.par_iter().for_each(|(spring, nums)| {
        let tx = tx.clone();
        // let pairs: Vec<&[Spring]> = spring.split(|s| s == &Spring::Operational).collect();

        let possible_springs = count_variants(&spring, nums);
        let sum = possible_springs;
        // println!(".");
        let _ = tx.send(sum);
    });
    drop(tx);
    let mut totsum = 0;
    while let Ok(sum) = rx.recv() {
        totsum += sum;
    }
    println!("sum: {totsum}");
}
fn parse_input(input: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
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
#[cfg(test)]
mod tests {
    use crate::{count_variants, parse_input};

    const TESTINPUT: &str = "?###???????? 3,2,1";
    #[test]
    fn it_works() {
        let springs = parse_input(TESTINPUT);
        for (spring, count) in springs {
            let num = count_variants(&spring[0..12], &count[0..3]);
            println!("{num}");
        }

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
