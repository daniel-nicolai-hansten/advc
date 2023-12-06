use std::{fs, num::ParseIntError};

fn main() {
    // let input = TESTINPUT;
    let input = fs::read_to_string("input.txt").unwrap();
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
            "seed-to-soil map" => ParseState::Soil,
            "soil-to-fertilizer map" => ParseState::Fertilizer,
            "fertilizer-to-water map" => ParseState::Water,
            "water-to-light map" => ParseState::Light,
            "light-to-temperature map" => ParseState::Temperature,
            "temperature-to-humidity map" => ParseState::Humidity,
            "humidity-to-location map" => ParseState::Location,
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
    let mut lowest_loc = 0;
    for seed in seeds {
        let soil = find_in_rangemaps(&seed_to_soil, seed);
        let fertilizer = find_in_rangemaps(&soil_to_fertilizer, soil);
        let water = find_in_rangemaps(&fertilizer_to_water, fertilizer);
        let light = find_in_rangemaps(&water_to_light, water);
        let temp = find_in_rangemaps(&light_to_temperature, light);
        let humidity = find_in_rangemaps(&temperature_to_humidity, temp);
        let location = find_in_rangemaps(&humidity_to_location, humidity);
        if lowest_loc != 0 {
            if location < lowest_loc {
                lowest_loc = location;
            }
        } else {
            lowest_loc = location;
        }
    }

    println!("{lowest_loc}");
}
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
        let nums: Vec<Result<u64, ParseIntError>> = value
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>())
            .collect();

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
