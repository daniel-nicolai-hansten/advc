use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bits::complete::take, bytes::complete::take as takeb, character::complete::hex_digit1, multi::many1, IResult, Parser};
#[aoc_generator(day16)]
fn parse(input: &str) -> Packet {
    let (_rest, data) = hex_to_bytes(input).unwrap();
    let package = Packet::from_bits((&data, 0)).unwrap();
    package.1
}

fn hex_to_bytes(input: &str) -> IResult<&str, Vec<u8>> {
    let (rest, data) = many1(takeb(2_usize).map(|s| u8::from_str_radix(s, 16).unwrap())).parse(input)?;
    Ok((rest, data))
}
#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    content: PacketContent,
}
impl Packet {
    fn from_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let (rest, version): ((&[u8], usize), u8) = take(3_usize).parse(input)?;
        let (rest, type_id): ((&[u8], usize), u8) = take(3_usize).parse(rest)?;
        let (rest, content) = PacketContent::from_bits(rest, type_id)?;
        Ok((rest, Packet { version, type_id, content }))
    }
    fn version_sum(&self) -> u64 {
        let mut sum = self.version as u64;
        if let PacketContent::Operator(subpackets) = &self.content {
            for sp in subpackets {
                
                sum += sp.version_sum();
            }
        }
        sum
    }
    fn display(&self, offset: usize) {
        print!("{}", 0..offset.map(|_|' '));
        match self.content {
            PacketContent::Literal(n) => println!("Package: Literal: {}",n),
            PacketContend::Operator => {println!("Package: Operator"); &self.display(offset +2);
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum PacketContent {
    Literal(u64),
    Operator(Vec<Packet>),
}
impl PacketContent {
    fn from_bits(mut input: (&[u8], usize), type_id: u8) -> IResult<(&[u8], usize), Self> {
        match type_id {
            4 => {
                let mut value = 0u64;
                loop {
                    let (rest_tmp, prefix): ((&[u8], usize), u8) = take(1_usize).parse(input)?;
                    let (rest_tmp, value_chunk): ((&[u8], usize), u64) = take(4_usize).map(|v: u64| v).parse(rest_tmp)?;
                    value = (value << 4) | value_chunk;
                    input = rest_tmp;
                    if prefix == 0 {
                        break;
                    }
                }
                Ok((input, PacketContent::Literal(value)))
            }
            _operator => {
                let mut ret = vec![];
                let (rest, length_type_id): ((&[u8], usize), u8) = take(1_usize).parse(input)?;
                input = rest;
                match length_type_id {
                    0 => {
                        let (rest, total_bits): ((&[u8], usize), u64) = take(15_usize).parse(input)?;
                        input = rest;
                        let target_bitpos = (input.0.len() * 8 - input.1) - (total_bits as usize);
                        let mut subpacket_rest = input;
                        while (subpacket_rest.0.len() * 8 - subpacket_rest.1) > target_bitpos {
                            let (new_rest, packet) = Packet::from_bits(subpacket_rest)?;
                            ret.push(packet);
                            subpacket_rest = new_rest;
                        }
                    }
                    1 => {
                        let (rest, total_packets): ((&[u8], usize), u64) = take(11_usize).parse(input)?;
                        input = rest;
                        let mut subpacket_rest = input;
                        for _ in 0..total_packets as usize {
                            let (new_rest, packet) = Packet::from_bits(subpacket_rest)?;
                            ret.push(packet);
                            subpacket_rest = new_rest;
                        }
                        input = subpacket_rest;
                    }
                    _ => unreachable!(),
                }

                // Parse operator packet
                Ok((input, PacketContent::Operator(ret)))
            }
        }
    }
}
#[aoc(day16, part1)]
fn part1(input: &Packet) -> u64 {
    input.version_sum()
}

// #[aoc(day16, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    const PKG1: &str = "D2FE28";
    const PKG2: &str = "38006F45291200";
    const PKG3: &str = "EE00D40C823060";

    #[test]
    fn part1_example() {
        let pkg = parse("C0015000016115A2E0802F182340");
        println!("Parsed package: {:?}", pkg);
        // assert_eq!(part1(&parse(PKG1)), 6);
        // assert_eq!(part1(&parse(PKG2)), 9);
        // assert_eq!(part1(&parse(PKG3)), 14);
        assert_eq!(part1(&parse("8A004A801A8002F478")), 16);
        // assert_eq!(part1(&parse("C0015000016115A2E0802F182340")), 23);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(PKG1)), "<RESULT>");
    // }
}
