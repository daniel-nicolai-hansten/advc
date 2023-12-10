use std::collections::HashMap;

use itertools::Itertools;
fn main() {
    let input = TESTINPUT;
    let mut segment: Vec<String> = input
        .split_ascii_whitespace()
        .filter(|c| *c != "|")
        .map(|c| c.chars().sorted().collect::<String>())
        .collect();
    segment.sort_unstable();
    segment.dedup();
    let mut map = HashMap::new();
    loop {
        for seg in &segment {
            match seg.len() {
                2 => map.insert(SegNum::One, seg.as_str()),
                3 => map.insert(SegNum::Seven, seg.as_str()),
                7 => map.insert(SegNum::Eight, seg.as_str()),
                _ => None,
            };
        }
    }

    println!("{:?}", segment);
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SegNum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Unknown,
}
const TESTINPUT: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf";
