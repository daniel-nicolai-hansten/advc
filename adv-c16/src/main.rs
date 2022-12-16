#[allow(unused_variables)]
use std::fs;
use std::process::exit;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools; 

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let valves_map = parse_input(&input);
    let mut sulotions: Vec<usize> = vec![];
    println!("steps to DD {}", bfs_calculate_steps(&valves_map, usize::from_str_radix("AA", 36).unwrap(), usize::from_str_radix("DD", 36).unwrap()));
    let working_valves = find_working_valves(&valves_map);
    //println!("per {}", working_valves.iter().permutations(working_valves.len()).len());
    for path in working_valves.iter().permutations(working_valves.len()) {
        let possible_flow = calculate_flow(&path, &valves_map);
        sulotions.push(possible_flow);
        //println!("{:?}, {}", path, possible_flow);
    }
    sulotions.sort();
    println!("{:?}", sulotions.pop().unwrap());
    
}

fn calculate_flow(valvelist: &Vec<&usize>, map: &HashMap<usize, Valve>) -> usize {
    let mut time = 30;
    let mut flow = 0;
    let mut pos = usize::from_str_radix("AA", 36).unwrap();
    for valve in valvelist {
        let traveltime = bfs_calculate_steps(map, pos, **valve);
        if time < traveltime {
            break;
        }
        time -= traveltime;
        pos = **valve;
        flow += time * map.get(&valve).unwrap().flow_rate;
    }
    flow

}

fn find_working_valves(map: &HashMap<usize, Valve>) -> Vec<usize> {
    let mut res = vec![];
    for (_id, valve) in map {
        if valve.flow_rate > 0{
            res.push(valve.id);
        } 
    }
    res
}

#[derive(Debug)]
struct Valve {
    id: usize,
    flow_rate: usize,
    connected_to: Vec<usize>,
}
fn parse_input(input: &str) -> HashMap<usize, Valve> {
    // let mut valves = vec![];
    let mut valves = HashMap::new();
    for line in input.lines() {
        let splitline: Vec<&str> = line.split(";").collect();
        let valve: Vec<&str> = splitline[0]
            .strip_prefix("Valve ")
            .unwrap()
            .split(" has flow rate=")
            .collect();
        let valves_connected: Vec<&str> = splitline[1].split(" ").collect();
        let mut valves_connected_vec = vec![];
        for (i, connection) in valves_connected.iter().enumerate() {
            if i > 4 {
                valves_connected_vec
                    .push(usize::from_str_radix(connection.trim_end_matches(","), 36).unwrap());
            }
        }
        let current_valve = Valve {
            id: usize::from_str_radix(valve[0], 36).unwrap(),
            flow_rate: valve[1].parse().unwrap(),
            connected_to: valves_connected_vec,
        };
        println!("{:?}", current_valve);
        valves.insert(current_valve.id, current_valve);
    }
    valves
}
fn bfs_calculate_steps(map: &HashMap<usize, Valve>, start: usize, target: usize) -> usize {
    let mut que = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();
    que.push_back(start);
    let mut steps = 0;
    'outer: while steps < 30{
        steps += 1;
        for i in 0..que.len(){
            let node = que.pop_front().unwrap();
            visited.insert(node);
            if node == target {
                break 'outer;
            }
            for neighbor in map.get(&node).unwrap().connected_to.clone() {
                if !visited.contains(&neighbor){
                    que.push_back(neighbor);
                }
                
            }
        }
    }
    
    steps
} 
const TESTINPUT: &str = 
"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
