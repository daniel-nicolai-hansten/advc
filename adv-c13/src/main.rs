use std::clone;
use std::collections::HashSet;
use std::collections::VecDeque;
#[allow(unused_variables)]
use std::fs;
use std::slice::Iter;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut chars_recived: [Vec<PacketData>; 2] = [vec![], vec![]];
    let mut index = 0;
    let mut results: Vec<u32> = vec![];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {}
        if i % 3 == 2 {
            if is_inorder(&chars_recived[0], &chars_recived[1]) {
                results.push(index + 1);
            }
            index += 1;
            println!("index: {}", index)
        } else {
            chars_recived[i % 3] = parse_line_to_vec(line);
        }
    }
    println!("{},  {:?}", results.iter().sum::<u32>(), results);
    //println!("{:?}", chars_recived[1]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PacketData {
    c: Option<u32>,
    depth: usize,
    array_place: u32,
}
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}
fn parse_line_to_vec(line: &str) -> Vec<Packet> {
    let mut depth = 0;
    let mut empty_list = false;
    let mut array_placement_list = [0; 64];
    let mut ret = vec![];
    for c in line.chars() {
        match c {
            '[' => {
                empty_list = true;
                depth += 1;
                array_placement_list[depth] = 0;
            }
            ']' => {
                if empty_list {
                    ret.push(PacketData {
                        c: None,
                        depth,
                        array_place: array_placement_list[depth],
                    });
                }
                //array_place = 0;
                depth -= 1;
            }
            ',' => array_placement_list[depth] += 1,
            _ => {
                ret.push(PacketData {
                    c: Some(c.to_digit(16).unwrap()),
                    depth,
                    array_place: array_placement_list[depth],
                });
                empty_list = false;
            }
        }
    }
    ret
}
fn is_inorder(pkt0_l: &Vec<PacketData>, pkt1_l: &Vec<PacketData>) -> bool {
    let mut inorder = None;
    let mut i = 0;
    'outer: loop {
        if pkt0_l.len() == i || pkt1_l.len() == i {
            if pkt0_l.len() < pkt1_l.len() {
                inorder = Some(true);
            } else if pkt0_l.len() > pkt1_l.len() {
                inorder = Some(false);
            }
            break 'outer;
        }
        let mut pkt0 = pkt0_l[i].clone();
        let mut pkt1 = pkt1_l[i].clone();
        if pkt0.depth != pkt1.depth {
            if (pkt0.depth > pkt1.depth) && pkt0.array_place == 0 {
                // comparing int and array
                pkt1.depth = pkt0.depth;
                pkt1.array_place = 0;
            } else if (pkt1.depth > pkt0.depth) && pkt1.array_place == 0 {
                // comparing int and array
                pkt0.depth = pkt1.depth;
                pkt0.array_place = 0;
            }
        }
        if pkt0 != pkt1 {
            if pkt0.depth == pkt1.depth && pkt0.array_place == pkt1.array_place {
                // only value diffrent
                if pkt0.c < pkt1.c {
                    inorder = Some(true);
                    break 'outer;
                } else if pkt0.c > pkt1.c {
                    inorder = Some(false);
                    break 'outer;
                }
            } else if pkt0.depth > pkt1.depth {
                inorder = Some(false);
                break 'outer;
            } else if pkt1.depth > pkt0.depth {
                inorder = Some(true);
                break 'outer;
            } else if pkt0.array_place == 0 && pkt1.array_place != 0 {
                inorder = Some(true);
                break 'outer;
            } else if pkt1.array_place == 0 && pkt0.array_place != 0 {
                inorder = Some(false);
                break 'outer;
            } else {
                println!("Err parsing lines2:");
                println!("{:?}", pkt0);
                println!("{:?}", pkt1);
            }
        }
        i += 1;
    }
    if !inorder.is_some() {
        println!("Err parsing lines3:");
        println!("{:?}", pkt0_l);
        println!("{:?}", pkt1_l);
    }
    inorder.unwrap()
}
#[cfg(test)]
mod day_13_tests {
    use super::*;
    #[test]
    fn test_parse_line_to_vec() {
        let ln1 = parse_line_to_vec("[1,1,3,1,1]");
        let ln2 = parse_line_to_vec("[1,1,5,1,1]");
        assert_ne!(&ln1, &ln2);
    }
    #[test]
    fn test_is_inorder1() {
        let ln1 = parse_line_to_vec("[1,1,3,1,1]");
        let ln2 = parse_line_to_vec("[1,1,5,1,1]");
        assert!(is_inorder(&ln1, &ln2));
        assert!(!is_inorder(&ln2, &ln1));
    }
    #[test]
    fn test_is_inorder2() {
        let ln1 = parse_line_to_vec("[[1],[2,3,4]]");
        let ln2 = parse_line_to_vec("[[1],4]");
        assert!(is_inorder(&ln1, &ln2));
        assert!(!is_inorder(&ln2, &ln1));
    }
    #[test]
    fn test_is_inorder3() {
        let ln1 = parse_line_to_vec("[9]");
        let ln2 = parse_line_to_vec("[[8,7,6]]");
        assert!(!is_inorder(&ln1, &ln2));
        assert!(is_inorder(&ln2, &ln1));
    }
    #[test]
    fn test_is_inorder4() {
        let ln1 = parse_line_to_vec("[[4,4],4,4]");
        let ln2 = parse_line_to_vec("[[4,4],4,4,4]");
        assert!(is_inorder(&ln1, &ln2));
        assert!(!is_inorder(&ln2, &ln1));
    }
    #[test]
    fn test_is_inorder5() {
        let ln1 = parse_line_to_vec("[[4,4],4,4]");
        let ln2 = parse_line_to_vec("[[4,4,4]]");
        println!("{:?}", ln1);
        println!("{:?}", ln2);
        assert!(is_inorder(&ln1, &ln2));
        assert!(!is_inorder(&ln2, &ln1));
    }
    #[test]
    fn test_is_inorder6() {
        let ln1 = parse_line_to_vec("[[4,4],[4],4]");
        let ln2 = parse_line_to_vec("[[4,4,4]]");
        println!("{:?}", ln1);
        println!("{:?}", ln2);
        assert!(is_inorder(&ln1, &ln2));
        assert!(!is_inorder(&ln2, &ln1));
    }
    #[test]
    fn test_is_inorder7() {
        let ln1 = parse_line_to_vec("[[[]]]");
        let ln2 = parse_line_to_vec("[[]]");
        assert!(!is_inorder(&ln1, &ln2));
        assert!(is_inorder(&ln2, &ln1));
    }
    #[test]
    fn test_is_inorder8() {
        let ln1 = parse_line_to_vec("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let ln2 = parse_line_to_vec("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert!(!is_inorder(&ln1, &ln2));
        assert!(is_inorder(&ln2, &ln1));
    }
    #[test]
    fn test_is_inorder9() {
        let ln1 = parse_line_to_vec("[[[[6,6,1],[3,10,5]],[[3,9,6,0],2,[10,1,8,8]],[],[[6,9],5,5,[],4]],[[7],3],[[[],[8,4,2,0,8]],10,2,7],[8],[10,7,[[2,6,0,0,8],[3,10,10],[],9,[7,9,3]],6]]");
        let ln2 =
            parse_line_to_vec("[[6,8,[[9,0,8,7,5],0,3],[],0],[8,[6,9],8,[[0],7,[10,6],6,[7]],8]]");
        assert!(!is_inorder(&ln1, &ln2));
        assert!(is_inorder(&ln2, &ln1));
    }
}
const TESTINPUT: &str = "[1,1,3,1,1]
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

[[7]]
[[7,[[4,0,9,9]],3,[10,0,6,[5,2,0,1,7]],7],[5,[3,[2,8,7],[9],[2,3,5,0,10]],10,[[3,0,8],6,[5,5,7],[2,10]]],[3,4]]

[[[[5,10,10,8]],[[5,7,10,10],[],[2,4,10,9],1,[]],9,8,[10,[5],[7,6,8,2],10,[2,9]]],[8,3,[[],3,[4,3,0],1,2],[6,[2,8],[2],0,0],5],[2,[[6,0,0,4,9],[7,10],1,[]],[[8],10],[[1,10]]],[6,10,[[3,6,7,9],5],4]]
[[[]],[[[10,5,9],2],4,[0,[8],[0,8,9,2,6]],[8,0,[4,4,10],7]],[[6,[]],10,[[],[5,10]],[6,9],[3,10,[2,5,1,7,7],[2,6,10]]],[[],[[],5],[[5],[7],[8,2,2,8]],[2,[4,0],8]],[[[10],8,[10]],[],[]]]

[[[[6,6,1],[3,10,5]],[[3,9,6,0],2,[10,1,8,8]],[],[[6,9],5,5,[],4]],[[7],3],[[[],[8,4,2,0,8]],10,2,7],[8],[10,7,[[2,6,0,0,8],[3,10,10],[],9,[7,9,3]],6]]
[[6,8,[[9,0,8,7,5],0,3],[],0],[8,[6,9],8,[[0],7,[10,6],6,[7]],8]]

[[[[9],[6,6],0,8,[7,2]],[7,[],[9,6,2],9,[]],[[],7,7,[6,6]]],[1,3,1],[1,[[8,0,10,1,9]],9],[10,5]]
[[[[4,6,8,9],8,[3,3,6],[10]],[[3,6,7,5],[],[4,9]],[6,0,8],[],[5,[2]]],[[[5,1,6,2,8],2],3,[9,9,[],0,1],5,[10,[9,5,2],5]],[]]";
