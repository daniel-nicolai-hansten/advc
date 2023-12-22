use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<RangeMap> {
    let mut seeds = vec![];
    let mut seed_to_soil = vec![];
    let mut soil_to_fertilizer = vec![];
    let mut fertilizer_to_water = vec![];
    let mut water_to_light = vec![];
    let mut light_to_temperature = vec![];
    let mut temperature_to_humidity = vec![];
    let mut humidity_to_location = vec![];
    let mut state = ParseState::Seeds;
    for line in input.lines() {
        let splits: Vec<&str> = line.split(":").collect();
        state = match splits[0] {
            "seeds" => ParseState::Seeds,
            "seed2soil map" => ParseState::Soil,
            "soil2fertilizer map" => ParseState::Fertilizer,
            "fertilizer2water map" => ParseState::Water,
            "water2light map" => ParseState::Light,
            "light2temperature map" => ParseState::Temperature,
            "temperature2humidity map" => ParseState::Humidity,
            "humidity2location map" => ParseState::Location,
            _ => state,
        };
        match state {
            ParseState::Seeds => {
                if splits.len() > 1 {
                    seeds = splits[1]
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect()
                }
            }
            ParseState::Soil => {
                if let Some(range) = RangeMap::try_from(splits[0]) {
                    seed_to_soil.push(range);
                }
            }
            ParseState::Fertilizer => {
                if let Some(range) = RangeMap::try_from(splits[0]) {
                    soil_to_fertilizer.push(range);
                }
            }
            ParseState::Water => {
                if let Some(range) = RangeMap::try_from(splits[0]) {
                    fertilizer_to_water.push(range);
                }
            }
            ParseState::Light => {
                if let Some(range) = RangeMap::try_from(splits[0]) {
                    water_to_light.push(range);
                }
            }
            ParseState::Temperature => {
                if let Some(range) = RangeMap::try_from(splits[0]) {
                    light_to_temperature.push(range);
                }
            }
            ParseState::Humidity => {
                if let Some(range) = RangeMap::try_from(splits[0]) {
                    temperature_to_humidity.push(range);
                }
            }
            ParseState::Location => {
                if let Some(range) = RangeMap::try_from(splits[0]) {
                    humidity_to_location.push(range);
                }
            }
        }
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    todo!()
}
type Range = (u32, u32);
enum RangeMap {
    Seed2Soil(Range),
    soil2fertilizer(Range),
    fertilizer2water(),
    water2light(),
    light2temperature(),
    temperature2humidity,

    humidity2location,
}
use rayon::prelude::*;
use std::time::Instant;
use std::{fs, num::ParseIntError, sync::mpsc};

// fn main() {
//     let mut lowest_loc = u64::MAX;
//     for seed in &seeds {
//         let soil = find_in_rangemaps(&seed_to_soil, *seed);
//         let fertilizer = find_in_rangemaps(&soil_to_fertilizer, soil);
//         let water = find_in_rangemaps(&fertilizer_to_water, fertilizer);
//         let light = find_in_rangemaps(&water_to_light, water);
//         let temp = find_in_rangemaps(&light_to_temperature, light);
//         let humidity = find_in_rangemaps(&temperature_to_humidity, temp);
//         let location = find_in_rangemaps(&humidity_to_location, humidity);

//         if location < lowest_loc {
//             lowest_loc = location;
//         }
//     }
//     let elapsed1 = now.elapsed();
//     println!("p1 lowest: {lowest_loc}");
//     println!("Elapsed: {:.2?}", elapsed1);

//     let mut seed_ranges = vec![];
//     let (tx, rx) = mpsc::channel();
//     let mut tot_rane = 0;
//     for seed in seeds.chunks(2) {
//         tot_rane += seed[1];
//         seed_ranges.push((seed[0], seed[1], tx.clone()));
//     }
//     println!("Total seeds: {tot_rane}");
//     seed_ranges.par_iter().for_each(|(seed_s, range, tx)| {
//         let mut lowest_loc = u64::MAX;
//         // println!("{seed_s}");
//         for i in 0..*range {
//             let seed = seed_s + i;
//             let soil = find_in_rangemaps(&seed_to_soil, seed);
//             let fertilizer = find_in_rangemaps(&soil_to_fertilizer, soil);
//             let water = find_in_rangemaps(&fertilizer_to_water, fertilizer);
//             let light = find_in_rangemaps(&water_to_light, water);
//             let temp = find_in_rangemaps(&light_to_temperature, light);
//             let humidity = find_in_rangemaps(&temperature_to_humidity, temp);
//             let location = find_in_rangemaps(&humidity_to_location, humidity);

//             if location < lowest_loc {
//                 lowest_loc = location;
//             }
//         }
//         tx.send(lowest_loc).unwrap();
//         // println!("lowest: {lowest_loc}");
//     });
//     drop(tx);
//     drop(seed_ranges);
//     let mut result = u64::MAX;
//     while let Ok(lw) = rx.recv() {
//         if result > lw {
//             result = lw;
//         }
//     }
//     let elapsed2 = now.elapsed();
//     println!("p2 lowest: {result}");
//     println!("Elapsed: {:.2?}", elapsed2);
// }
#[derive(Debug)]
enum ParseState {
    Seeds,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
#[derive(Debug)]
struct RangeMap {
    s_start: u64,
    s_end: u64,
    d_start: u64,
    d_end: u64,
}

impl RangeMap {
    fn try_from(value: &str) -> Option<Self> {
        let nums: Vec<Result<u64, ParseIntError>> = value.split_ascii_whitespace().map(|x| x.parse::<u64>()).collect();

        let range = nums.get(2)?.clone().ok()?;
        let d_start = nums.get(0)?.clone().ok()?;
        let s_start = nums.get(1)?.clone().ok()?;
        let s_end = s_start + range;
        let d_end = d_start + range;
        Some(Self {
            s_start,
            s_end,
            d_start,
            d_end,
        })
    }
    fn in_range(&self, s: u64) -> Option<u64> {
        let mut ret = None;
        if s >= self.s_start && s < self.s_end {
            let diff = s - self.s_start;
            ret = Some(self.d_start + diff);
        }
        ret
    }
}
fn find_in_rangemaps(maps: &Vec<RangeMap>, s: u64) -> u64 {
    let mut ret = s;
    for map in maps {
        if let Some(d) = map.in_range(s) {
            ret = d;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "seeds: 79 14 55 13

    seed2soil map:
    50 98 2
    52 50 48
    
    soil2fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer2water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water2light map:
    88 18 7
    18 25 70
    
    light2temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature2humidity map:
    0 69 1
    1 0 69
    
    humidity2location map:
    60 56 37
    56 93 4";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
