use std::collections::HashSet;
use std::collections::VecDeque;
#[allow(unused_variables)]
use std::fs;
use std::slice::Iter;
const H: usize = 41;
const W: usize = 136;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut chars_recived: [Vec<PacketData>; 2] = [vec![], vec![]];
    let mut index = 0;
    let mut results = vec![];
    for (i, line) in TESTINPUT.lines().enumerate() {
        let mut depth = 0;
        let mut empty_list = false;
        for (j, c) in line.chars().enumerate() {
            match c {
                '[' => {
                    empty_list = true;
                    depth += 1;
                }
                ']' => {
                    if empty_list {
                        chars_recived[i % 3].push(PacketData {
                            c: None,
                            depth: depth,
                        });
                    }
                    depth -= 1;
                }
                ',' => (),
                _ => {
                    chars_recived[i % 3].push(PacketData {
                        c: Some(c.to_digit(16).unwrap()),
                        depth: depth,
                    });
                    empty_list = false;
                }
            }
        }
        if i % 3 == 2 {
            let mut outoforder = false;
            for i in 0..chars_recived[0].len() {
                let pkt0 = &chars_recived[0][i];
                let pkt1 = &chars_recived[1][i];
                if pkt0 != pkt1 {
                    if pkt0.depth == pkt1.depth {
                        if pkt0.c > pkt1.c {
                            outoforder = true;
                            break;
                        }
                    } else if {

                    }
                    println!("{:?}", chars_recived[0][i]);
                }
            }
            if !outoforder {
                results.push(index +1);
            }
            index += 1;
            chars_recived[0].clear();
            chars_recived[1].clear();
            //break;
        };
    }
    println!("{:?}", results);
    //println!("{:?}", chars_recived[1]);
}
#[derive(Debug, PartialEq, Eq)]
struct PacketData {
    c: Option<u32>,
    depth: u32,
}
const TESTINPUT: &str = 
"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

#[cfg(test)]
mod day_13_tests {
    use super::*;
    #[test]
    fn test_simple_order() {}

}