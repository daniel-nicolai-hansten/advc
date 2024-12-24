use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<u8> {
    input.split(",")
        .map(|x|  x.parse::<u8>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> u64 {
    run_sim(input, 80).unwrap()
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> u64 {
    run_sim(input, 256).unwrap()
}

fn run_sim(input: &[u8], simlen: u32) -> Option<u64>{
    let mut fishies: VecDeque<u64>  = [0;9].into();
    for fish in input {
        *fishies.get_mut(*fish as usize)? += 1;
    }
    for _i in 0..simlen {
        let fish = fishies.pop_front()?;
        *fishies.get_mut(6)? += fish;
        fishies.push_back(fish);
    }
    Some(fishies.iter().sum())
}


#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "3,4,3,1,2";
    #[test]
    fn part1_example() {
        assert_eq!(run_sim(&parse(TESTINPUT), 18), Some(26));
        assert_eq!(part1(&parse(TESTINPUT)), 5934);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 26984457539);
    }
}