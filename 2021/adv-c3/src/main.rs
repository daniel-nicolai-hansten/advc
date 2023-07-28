use std::{fs, isize};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut counters = [0; 12];
    let mut nums = Vec::new();
    for line in input.lines() {
        let mut i = 0;
        for c in line.chars() {
            match c {
                '0' => counters[i] -= 1,
                '1' => counters[i] += 1,
                _ => {}
            }
            i += 1;
        }
        nums.push(u32::from_str_radix(line, 2).unwrap());
    }

    println!("{:?}", counters);

    let (mut oxygen_generator_rating, mut CO2_scrubber_rating) = (0,0);
    let mut currentnums = nums.clone();
    for i in 0..LEN {
        if !calculate_bit_criteria(&currentnums, i) {
            currentnums.retain(|x| *x & (1 << LEN -1 - i) == 0);
        } else {
            currentnums.retain(|x| *x & (1 << LEN -1 - i) != 0);
        }
        if currentnums.len() == 1 {
            oxygen_generator_rating = currentnums[0];
            break;
        }
    }
    let mut currentnums = nums.clone();
    for i in 0..LEN {
        if !calculate_bit_criteria(&currentnums, i) {
            currentnums.retain(|x| *x & (1 << LEN -1 - i) != 0);
        } else {
            currentnums.retain(|x| *x & (1 << LEN -1 - i) == 0);
        }
        if currentnums.len() == 1 {
            CO2_scrubber_rating = currentnums[0];
            break;
        }
    }
    println!("{}  {}  {}", oxygen_generator_rating, CO2_scrubber_rating, oxygen_generator_rating * CO2_scrubber_rating );
}

fn calculate_bit_criteria(list: &Vec<u32>, bitnum: u32) -> bool {
    let mut counters = 0;
    if bitnum > 12 {
        panic!();
    }
    for i in list {
        if i & (1 << LEN -1 - bitnum) == 0 {
            counters -= 1;
        } else {
            counters += 1;
        }
    }
    counters >= 0
}
const LEN: u32 = 12;
const TESTINPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
