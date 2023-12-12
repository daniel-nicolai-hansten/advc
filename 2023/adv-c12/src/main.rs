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
        '.' => Spring::Operational,
        '?' => Spring::Unknown,
        _ => Spring::Err,
    };
    for line in input.lines() {
        let splits: Vec<&str> = line.trim_start().split_ascii_whitespace().collect();
        let gearstates: Vec<Spring> = splits[0].chars().map(gearsplit).collect();
        let nums: Vec<usize> = splits[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
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
    let n = springs.len();
    let m = counts.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];
    dp[n][m][0] = 1;
    dp[n][m - 1][counts[m - 1]] = 1;
    use Spring as S;
    for pos in (0..n).rev() {
        for (group, &max_count) in counts.iter().enumerate() {
            for count in 0..=max_count {
                for c in [S::Operational, S::Broken] {
                    if springs[pos] == c || springs[pos] == S::Unknown {
                        if c == S::Operational && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == S::Operational && group < m && counts[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == S::Broken {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
        if matches!(springs[pos], Spring::Operational | Spring::Unknown) {
            dp[pos][m][0] += dp[pos + 1][m][0];
        }
    }
    dp[0][0][0]
}

const TESTINPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        let input = "#.#.###. 1,1,3";
        let springs = parse_input(input);
        let (spring, num) = &springs[0];
        assert!(cmp_springs(spring, num));
    }
    // #[test]
    // fn it_works2() {
    //     let springs = parse_input(TESTINPUT);
    //     let mut sum = 0;
    //     for (spring, nums) in &springs {
    //         let possible_springs = generate_variance(&spring, &nums);
    //         for spring in possible_springs {
    //             if cmp_springs(&spring, nums) {
    //                 println!("spring: {spring:?}");
    //                 sum += 1;
    //             }
    //         }
    //     }
    //     println!("sum: {sum}");
    // }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Broken,
    Operational,
    Unknown,
    Err,
}
