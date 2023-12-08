use std::{fs, vec};

fn main() {
    // let input = TESTINPUT;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut parsestate = 0;
    let mut rl_input = "";
    let mut maps = vec![];
    for line in input.lines() {
        match parsestate {
            0 => {
                rl_input = line;
                parsestate += 1;
            }
            _ => {
                if line.len() > 2 {
                    let chars: Vec<char> = line.chars().collect();
                    let splits: Vec<&str> = line.split(" = ").collect();
                    let name = u64::from_str_radix(splits[0], 36).unwrap();
                    let splits: Vec<&str> = splits[1]
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(", ")
                        .collect();
                    let left = u64::from_str_radix(splits[0], 36).unwrap();
                    let right = u64::from_str_radix(splits[1], 36).unwrap();
                    maps.push(MapNode { name, left, right });
                }
            }
        }
    }
    maps.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    let mut current_pos = u64::from_str_radix("AAA", 36).unwrap();
    let target_pos = u64::from_str_radix("ZZZ", 36).unwrap();
    let mut steps = 0;
    'outer: loop {
        for c in rl_input.chars() {
            if current_pos == target_pos {
                break 'outer;
            }
            steps += 1;
            let idx = maps.binary_search_by(|a| a.name.cmp(&current_pos)).unwrap();
            let current_node = &maps[idx];
            println!("{current_node:?}");
            match c {
                'R' => current_pos = current_node.right,
                'L' => current_pos = current_node.left,
                _ => (),
            }
        }
    }
    println!("steps {steps}");
}
#[derive(Debug)]
struct MapNode {
    name: u64,
    left: u64,
    right: u64,
}

const TESTINPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
