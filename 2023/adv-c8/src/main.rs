use std::time::Instant;
use std::vec;
fn main() {
    // let input = TESTINPUT;
    let now = Instant::now();
    let input = include_str!("../input.txt");
    let mut parsestate = 0;
    let mut rl_input = vec![];
    let mut starts = vec![];
    let mut ends = vec![];
    let mut maps = vec![];
    for line in input.lines() {
        match parsestate {
            0 => {
                rl_input = line.chars().collect();
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
    let p1 = find_steps(
        &maps,
        &rl_input,
        u64::from_str_radix("AAA", 36).unwrap(),
        &vec![u64::from_str_radix("ZZZ", 36).unwrap()],
    );
    println!("p1: {} {:.2?}", p1, now.elapsed());
    let mut stepvec = vec![];
    for start in &starts {
        stepvec.push(find_steps(&maps, &rl_input, *start, &ends));
    }
    let p2 = stepvec.into_iter().reduce(|acc, x| lcm(acc, x)).unwrap();
    println!("p2: {} {:.2?}", p2, now.elapsed());
    
}
fn find_steps(maps: &Vec<MapNode>, rl_input: &Vec<char>, start: u64, ends: &Vec<u64>) -> u64 {
    let mut steps = 0;
    let mut current_pos = start;
    'outer: loop {
        for c in rl_input {
            if ends.contains(&current_pos) {
                break 'outer;
            }
            steps += 1;
            let idx = maps.binary_search_by(|a| a.name.cmp(&current_pos)).unwrap();
            let current_node = &maps[idx];
            match c {
                'R' => current_pos = current_node.right,
                'L' => current_pos = current_node.left,
                _ => (),
            }
        }
    }
    steps
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
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
