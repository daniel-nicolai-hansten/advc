use std::{fs, vec};

fn main() {
    // let input = TESTINPUT;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut parsestate = 0;
    let mut rl_input = "";
    let mut starts = vec![];
    let mut ends = vec![];
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
                    if chars[2] == 'Z' {
                        println!("{line}");
                    }
                    let splits: Vec<&str> = line.split(" = ").collect();
                    let name = u64::from_str_radix(splits[0], 36).unwrap();
                    let splits: Vec<&str> = splits[1]
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(", ")
                        .collect();
                    let left = u64::from_str_radix(splits[0], 36).unwrap();
                    let right = u64::from_str_radix(splits[1], 36).unwrap();
                    maps.push(MapNode { name, left, right });
                    match chars[2] {
                        'A' => starts.push(name),
                        'Z' => ends.push(name),
                        _ => (),
                    }
                }
            }
        }
    }
    maps.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    let mut current_poses = starts;
    let mut steps = 0;
    'outer: loop {
        for c in rl_input.chars() {
            let mut num_at_target = 0;
            'inner: for pos in &current_poses {
                if ends.iter().find(|x| x == &pos).is_some() {
                    num_at_target += 1;
                } else {
                    break 'inner;
                }
            }
            if num_at_target == current_poses.len() {
                break 'outer;
            }
            for current_pos in current_poses.iter_mut() {
                let idx = maps.binary_search_by(|a| a.name.cmp(&current_pos)).unwrap();
                let current_node = &maps[idx];
                // println!("{current_node:?}");
                match c {
                    'R' => *current_pos = current_node.right,
                    'L' => *current_pos = current_node.left,
                    _ => (),
                }
            }
            steps += 1;
        }
    }
    println!("steps {steps}");
}
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let rem = max % min;
        if rem == 0 {
            return min;
        }

        max = min;
        min = rem;
    }
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
