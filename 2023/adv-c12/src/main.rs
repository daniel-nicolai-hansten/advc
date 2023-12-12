use rayon::prelude::*;
use std::sync::mpsc;
fn main() {
    let input = include_str!("../input.txt");
    let springs = parse_input(input);
    let (tx, rx) = mpsc::channel();
    springs.par_iter().for_each(|(spring, nums)| {
        let tx = tx.clone();
        let possible_springs = generate_variance(&spring, &nums);
        let sum = possible_springs;
        println!(".");
        let _ = tx.send(sum);
    });
    drop(tx);
    let mut totsum = 0;
    while let Ok(sum) = rx.recv() {
        totsum += sum;
    }
    println!("sum: {totsum}");
}
fn parse_input(input: &str) -> Vec<(Vec<Spring>, Vec<u32>)> {
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
        let nums: Vec<u32> = splits[1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let mut gearstates_p2: Vec<Spring> = vec![];
        let mut nums_p2: Vec<u32> = vec![];
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
fn cmp_springs(springlist: &[Spring], nums: &[u32]) -> bool {
    let mut last = Spring::Unknown;
    let mut brokens = 0;
    let mut springs = vec![];
    for spring in springlist {
        use Spring as S;
        match (spring, last) {
            (S::Broken, _) => brokens += 1,
            (S::Operational, S::Broken) => {
                springs.push(brokens);
                brokens = 0;
            }
            _ => (),
        }
        last = *spring;
    }
    if brokens != 0 {
        springs.push(brokens);
    }
    // println!("nums {nums:?}, springs {springs:?}");
    nums == springs
}

fn generate_variance(springs: &[Spring], nums: &[u32]) -> usize {
    let unknowns = springs.iter().filter(|s| **s == Spring::Unknown).count();
    let two: u32 = 2;
    let combinations = two.pow(unknowns as u32);
    let mut ret = 0;

    for i in 0..combinations {
        let mut unknows_ret = 0;
        let replace_unknown = |s: &Spring| {
            let mask = 1 << unknows_ret;
            match (s, mask & i != 0) {
                (Spring::Unknown, true) => {
                    unknows_ret += 1;
                    Spring::Broken
                }
                (Spring::Unknown, false) => {
                    unknows_ret += 1;
                    Spring::Operational
                }
                _ => *s,
            }
        };
        let fixed_springs: Vec<Spring> = springs.iter().map(replace_unknown).collect();
        if cmp_springs(&fixed_springs, nums) {
            ret += 1;
        }
    }
    ret
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
    #[test]
    fn it_works2() {
        let springs = parse_input(TESTINPUT);
        let mut sum = 0;
        for (spring, nums) in &springs {
            let possible_springs = generate_variance(&spring, &nums);
            for spring in possible_springs {
                if cmp_springs(&spring, nums) {
                    println!("spring: {spring:?}");
                    sum += 1;
                }
            }
        }
        println!("sum: {sum}");
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Broken,
    Operational,
    Unknown,
    Err,
}
