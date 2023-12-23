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
fn part1(input: &(Vec<u64>, Vec<u64>)) -> usize {
    let (times, record_distances) = input;
    let mut num_of_better_times = vec![];
    for (i, time) in times.iter().enumerate() {
        let mut distances = vec![];
        let record = record_distances[i];
        for speed in 0..*time {
            let timeleft = time - speed;
            let distance = speed * timeleft;
            distances.push((distance, speed));
        }
        distances.sort_unstable();
        let better_times: Vec<(u64, u64)> = distances
            .into_iter()
            .filter(|(x, _)| x > &record)
            .map(|(x1, x2)| (x1, x2))
            .collect();
        // println!("better: {better_times:?}");
        num_of_better_times.push(better_times.len());
    }
    num_of_better_times.iter().product()
}

#[aoc(day6, part2)]
fn part2(input: &(Vec<u64>, Vec<u64>)) -> usize {
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
    let mut num_of_better_times = vec![];
    let mut distances = vec![];
    let record = new_record_distances;
    for speed in 0..newtime {
        let timeleft = newtime - speed;
        let distance = speed * timeleft;
        distances.push((distance, speed));
    }
    distances.sort_unstable();
    let better_times: Vec<(u64, u64)> = distances
        .into_iter()
        .filter(|(x, _)| x > &record)
        .map(|(x1, x2)| (x1, x2))
        .collect();
    num_of_better_times.push(better_times.len());

    num_of_better_times.iter().product()
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
}
