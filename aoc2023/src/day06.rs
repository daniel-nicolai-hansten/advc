use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse(file: &str) -> (Vec<u64>, Vec<u64>) {
    let input: Vec<&str> = file.lines().collect();
    let l1: Vec<&str> = input[0].split(": ").collect();
    let l2: Vec<&str> = input[1].split(": ").collect();
    let times: Vec<u64> = l1[1]
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let record_distances: Vec<u64> = l2[1]
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    (times, record_distances)
}

#[aoc(day6, part1)]
fn part1(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    let (times, record_distances) = input;
    let mut num_of_better_times = vec![];
    for (i, time) in times.iter().enumerate() {
        let record = record_distances[i];
        let better_times = solve(*time, record);
        num_of_better_times.push(better_times);
    }
    num_of_better_times.iter().product()
}

#[aoc(day6, part2)]
fn part2(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    let (times, record_distances) = input;
    let newtime = times
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let new_record_distances = record_distances
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    solve(newtime, new_record_distances)
}

fn solve(tim: u64, winning_distance: u64) -> u64 {
    let time = tim as f64;
    let distance = winning_distance as f64;
    let last = (time / 2.0) - ((time / 2.0).powi(2) - distance).sqrt();
    let first = (time / 2.0) + ((time / 2.0).powi(2) - distance).sqrt();
    let res = first.ceil() - last.floor();
    res as u64 - 1
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 71503);
    }
    #[test]
    fn solver_part2_example() {
        let res = solve(71530, 940200);
        assert_eq!(res, 71503);
    }
}
