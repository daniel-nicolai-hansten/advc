use std::{fs, vec};

fn main() {
    // let input: Vec<&str> = TESTINPUT.lines().collect();
    let file = fs::read_to_string("input.txt").unwrap();
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
    // println!("{times:?}  {record_distances:?}");
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
    let tot: usize = num_of_better_times.iter().product();
    println!("tot: {tot}");
}

const TESTINPUT: &str = "Time:      71530
Distance:  940200";
