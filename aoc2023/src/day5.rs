use std::{collections::VecDeque, num::ParseIntError, thread::LocalKey};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::range;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Seeds,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<RangeMap>, Vec<u64>) {
    let mut ret = vec![];
    let mut seeds = vec![];
    let mut state = State::Seeds;
    for line in input.lines() {
        let line = line.trim();
        let splits: Vec<&str> = line.split(":").collect();
        state = match splits[0] {
            "seeds" => State::Seeds,
            "seed-to-soil map" => State::Soil,
            "soil-to-fertilizer map" => State::Fertilizer,
            "fertilizer-to-water map" => State::Water,
            "water-to-light map" => State::Light,
            "light-to-temperature map" => State::Temperature,
            "temperature-to-humidity map" => State::Humidity,
            "humidity-to-location map" => State::Location,
            _ => state,
        };
        match state {
            State::Seeds => {
                if splits.len() > 1 {
                    seeds = splits[1]
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect()
                }
            }
            _ => {
                if let Some(range) = RangeMap::try_from(splits[0], state) {
                    ret.push(range);
                }
            }
        }
    }
    (ret, seeds)
}
type Seed = (u64, State);
#[aoc(day5, part1)]
fn part1(input: &(Vec<RangeMap>, Vec<u64>)) -> u64 {
    let (range_maps, seeds) = input;
    let mut wq: VecDeque<Seed> = VecDeque::new();
    let mut locations = vec![];
    for seed in seeds {
        wq.push_back((*seed, State::Seeds));
    }
    while !wq.is_empty() {
        if let Some((seed, state)) = wq.pop_front() {
            if state == State::Location {
                locations.push(seed);
                continue;
            }
            let next_state = match state {
                State::Seeds => State::Soil,
                State::Soil => State::Fertilizer,
                State::Fertilizer => State::Water,
                State::Water => State::Light,
                State::Light => State::Temperature,
                State::Temperature => State::Humidity,
                State::Humidity => State::Location,
                State::Location => State::Location,
            };
            let next_seed = find_in_rangemaps(range_maps, seed, state);
            wq.push_back((next_seed, next_state));
        }
    }
    locations.iter().min().unwrap().clone()
}
type SeedRange = (u64, u64, State);
#[aoc(day5, part2)]
fn part2(input: &(Vec<RangeMap>, Vec<u64>)) -> u64 {
    let (range_maps, seeds) = input;
    let seed_ranges: Vec<SeedRange> = seeds.iter().tuples().map(|(x, y)| (*x, *y, State::Seeds)).collect();
    let mut wq: VecDeque<SeedRange> = VecDeque::new();
    let mut locations = vec![];
    for seed in seed_ranges {
        wq.push_back(seed);
    }
    while !wq.is_empty() {
        if let Some((seed, range, state)) = wq.pop_front() {
            if state == State::Location {
                locations.push(seed);
                continue;
            }
            let next_seeds = range_in_rangemaps(range_maps, (seed, range, state));
            for (next_seed_s, next_seed_r, state) in next_seeds {
                let next_state = match state {
                    State::Seeds => State::Soil,
                    State::Soil => State::Fertilizer,
                    State::Fertilizer => State::Water,
                    State::Water => State::Light,
                    State::Light => State::Temperature,
                    State::Temperature => State::Humidity,
                    State::Humidity => State::Location,
                    State::Location => State::Location,
                };
                wq.push_back((next_seed_s, next_seed_r, next_state));
            }
        }
    }
    locations.iter().min().unwrap().clone()
}

type Range = (u64, u64, u64);
enum RangeMap {
    Seed2Soil(Range),
    Soil2Fertilizer(Range),
    Fertilizer2Water(Range),
    Water2Light(Range),
    Light2Temperature(Range),
    Temp2Humidity(Range),
    Humidity2Location(Range),
}

impl RangeMap {
    fn try_from(value: &str, state: State) -> Option<Self> {
        let nums: Vec<Result<u64, ParseIntError>> = value.split_ascii_whitespace().map(|x| x.parse()).collect();
        let range = nums.get(2)?.clone().ok()?;
        let dest = nums.get(0)?.clone().ok()?;
        let source = nums.get(1)?.clone().ok()?;
        let ret_range = (dest, source, range);
        let ret = match state {
            State::Soil => RangeMap::Seed2Soil(ret_range),
            State::Fertilizer => RangeMap::Soil2Fertilizer(ret_range),
            State::Water => RangeMap::Fertilizer2Water(ret_range),
            State::Light => RangeMap::Water2Light(ret_range),
            State::Temperature => RangeMap::Light2Temperature(ret_range),
            State::Humidity => RangeMap::Temp2Humidity(ret_range),
            State::Location => RangeMap::Humidity2Location(ret_range),
            _ => return None,
        };
        Some(ret)
    }
    fn range(&self) -> Range {
        match self {
            RangeMap::Seed2Soil(r) => r.clone(),
            RangeMap::Soil2Fertilizer(r) => r.clone(),
            RangeMap::Fertilizer2Water(r) => r.clone(),
            RangeMap::Water2Light(r) => r.clone(),
            RangeMap::Light2Temperature(r) => r.clone(),
            RangeMap::Temp2Humidity(r) => r.clone(),
            RangeMap::Humidity2Location(r) => r.clone(),
        }
    }
    fn in_range(&self, s: u64) -> Option<u64> {
        let mut ret = None;
        let (d_start, s_start, range) = self.range();
        if s >= s_start && s < s_start + range {
            let diff = s - s_start;
            ret = Some(d_start + diff);
        }
        ret
    }
}
fn find_in_rangemaps(maps: &[RangeMap], s: u64, state: State) -> u64 {
    use RangeMap as RM;
    use State as S;
    let mut ret = s;
    let filter_map: Vec<&RangeMap> = match state {
        S::Seeds => maps.iter().filter(|x| matches!(x, RM::Seed2Soil(_))).collect(),
        S::Soil => maps.iter().filter(|x| matches!(x, RM::Soil2Fertilizer(_))).collect(),
        S::Fertilizer => maps.iter().filter(|x| matches!(x, RM::Fertilizer2Water(_))).collect(),
        S::Water => maps.iter().filter(|x| matches!(x, RM::Water2Light(_))).collect(),
        S::Light => maps.iter().filter(|x| matches!(x, RM::Light2Temperature(_))).collect(),
        S::Temperature => maps.iter().filter(|x| matches!(x, RM::Temp2Humidity(_))).collect(),
        S::Humidity => maps.iter().filter(|x| matches!(x, RM::Humidity2Location(_))).collect(),
        S::Location => maps.iter().filter(|x| matches!(x, RM::Humidity2Location(_))).collect(),
    };
    for map in filter_map {
        if let Some(d) = map.in_range(s) {
            ret = d;
            break;
        }
    }
    ret
}
fn range_in_rangemaps(maps: &[RangeMap], s: SeedRange) -> Vec<SeedRange> {
    use RangeMap as RM;
    use State as S;
    let mut seeds = s;
    let mut ret = vec![];
    let filter_map: Vec<&RangeMap> = match seeds.2 {
        S::Seeds => maps.iter().filter(|x| matches!(x, RM::Seed2Soil(_))).collect(),
        S::Soil => maps.iter().filter(|x| matches!(x, RM::Soil2Fertilizer(_))).collect(),
        S::Fertilizer => maps.iter().filter(|x| matches!(x, RM::Fertilizer2Water(_))).collect(),
        S::Water => maps.iter().filter(|x| matches!(x, RM::Water2Light(_))).collect(),
        S::Light => maps.iter().filter(|x| matches!(x, RM::Light2Temperature(_))).collect(),
        S::Temperature => maps.iter().filter(|x| matches!(x, RM::Temp2Humidity(_))).collect(),
        S::Humidity => maps.iter().filter(|x| matches!(x, RM::Humidity2Location(_))).collect(),
        S::Location => maps.iter().filter(|x| matches!(x, RM::Humidity2Location(_))).collect(),
    };
    for map in filter_map {
        let (_dest, source, range) = map.range();
        let (seed_start, seed_range, state) = seeds;
        let seed_end = seed_start + seed_range;
        if source >= seed_start && source < seed_end {
            let new_seed = (map.in_range(source).unwrap(), seed_end - source, state);
            seeds.1 = source - seed_start;
            ret.push(new_seed);
        } else if seed_start >= source && seed_start < source + range {
            if seed_end > source + range {
                let new_seed = (map.in_range(seed_start).unwrap(), range, state);
                seeds.0 = source + range;
                seeds.1 = seed_end - seeds.0;
                ret.push(new_seed);
            } else {
                let new_seed = (map.in_range(seed_start).unwrap(), seed_end - seed_start, state);
                seeds.0 = seed_end;
                seeds.1 = 0;
                ret.push(new_seed);
                break;
            }
        }
    }
    if seeds.1 > 0 {
        ret.push(seeds);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 46);
    }
}
