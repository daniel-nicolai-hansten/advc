use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::IResult;
#[aoc_generator(day16)]
fn parse(input: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (c1, c2) in input.chars().tuples() {
        let d1 = c1.to_digit(16).unwrap() as u8;
        let d2 = c2.to_digit(16).unwrap() as u8;
        bytes.push(d1 << 4 | d2);
    }
    bytes
}
fn pkg4(input: &str) -> IResult<&str, Vec<u32>> {
    todo!()
}
#[derive(Debug)]
struct Pkg {
    ver: u8,
    t_id: u8,
    dt: u64
}
impl Pkg {
    fn prse(data: &[u8], offset: usize) -> Self {
        let mut offset = offset;
        let mut data = data;
        let ver = get_bits(data, 3, offset);
        offset += 3;
        let t_id = get_bits(data, 3, offset);
        offset += 3;
        let mut dt: u64 = 0;
        loop {
            if offset > 8 {
                data = &data[1..];
                offset %= 8;                
            }
            println!("{offset}");
            let block = get_bits(data, 5, offset);
            dt <<= 4;
            dt |= (block & 0x0f) as u64;
            if block & 1 << 4 == 0 {
                break;
            }
            offset += 5;
        }
        Self { ver, t_id, dt }
    }
}
#[aoc(day16, part1)]
fn part1(input: &[u8]) -> String {
    todo!()
}

#[aoc(day16, part2)]
fn part2(input: &[u8]) -> String {
    todo!()
}
fn get_bits(data: &[u8], len: usize, offset: usize) -> u8 {
    const MSB: u16 = 0x1 << 15;
    let mut mask = 0;
    for i in 0..len {
        mask |= MSB >> i;
    }
    mask >>= offset;
    match data {
        &[d1] => {
            mask >>= 8;
            d1 & mask as u8
        }
        &[d1, d2, ..] => {
            let mut dt = u16::from_be_bytes([d1, d2]) & mask;
            dt >>= 16 - (len + offset);
            dt as u8
        }
        &[] => 0,
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "D2FE28";
    #[test]
    fn part1_example() {
        let dt = parse(TESTINPUT);
        let pkg = Pkg::prse(&dt, 0);
        println!("{pkg:?}");

        // assert_eq!(part1(&parse(TESTINPUT)), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        // assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
