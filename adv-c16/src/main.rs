use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_variables)]
use std::fs;
use std::process::exit;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let valves_map = parse_input(TESTINPUT);
    println!(
        "steps to DD {}",
        bfs_calculate_steps(
            &valves_map,
            usize::from_str_radix("AA", 36).unwrap(),
            usize::from_str_radix("DD", 36).unwrap()
        )
    );
    let mut working_valves = find_working_valves(&valves_map);
    let mut path = vec![];
    let mut time_left = 30;
    let mut pos = usize::from_str_radix("AA", 36).unwrap();
    while time_left > 0 {
        let (next_move, _) = find_next_best_move(
            &valves_map,
            pos,
            time_left,
            working_valves.clone().into_iter().collect(),
        );
        path.push(next_move);
        let next_move_steps = bfs_calculate_steps(&valves_map, pos, next_move);
        if time_left < next_move_steps {
            break;
        }
        time_left -= next_move_steps;
        pos = next_move;
        working_valves.remove(&next_move);
        println!(
            "Move: {} steps {}",
            valves_map.get(&next_move).unwrap().name,
            next_move_steps
        );
    }

    println!(
        "total flow for path {}   {:?}",
        calculate_flow(
            &path,
            usize::from_str_radix("AA", 36).unwrap(),
            &valves_map,
            30
        ),
        path
    );
}

fn calculate_flow2(
    valvelist: &Vec<&usize>,
    startpos: usize,
    map: &HashMap<usize, Valve>,
    time_left: usize,
) -> usize {
    let mut time = time_left;
    let mut flow = 0;
    let mut pos = startpos;
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
fn calculate_flow(
    valvelist: &Vec<usize>,
    startpos: usize,
    map: &HashMap<usize, Valve>,
    time_left: usize,
) -> usize {
    let mut time = time_left;
    let mut flow = 0;
    let mut pos = startpos;
    for valve in valvelist {
        let traveltime = bfs_calculate_steps(map, pos, *valve);
        if time < traveltime {
            break;
        }
        time -= traveltime;
        pos = *valve;
        flow += time * map.get(&valve).unwrap().flow_rate;
    }
    flow
}
fn find_next_best_move(
    map: &HashMap<usize, Valve>,
    pos: usize,
    time_left: usize,
    valves_left: Vec<usize>,
) -> (usize, usize) {
    let mut sulotions: Vec<(usize, usize)> = vec![];
    let mut permut = 9;
    if valves_left.len() <= permut {
        permut = valves_left.len();
    }

    for path in valves_left.iter().permutations(permut) {
        let possible_flow = calculate_flow2(&path, pos, map, time_left);
        sulotions.push((*path[0], possible_flow));
        //println!("{:?}, {}", path, possible_flow);
    }
    sulotions.sort();
    let res = sulotions.pop().unwrap();
    println!("{:?}", res);
    res
}
fn find_working_valves(map: &HashMap<usize, Valve>) -> HashSet<usize> {
    let mut res = HashSet::new();
    for (_id, valve) in map {
        if valve.flow_rate > 0 {
            res.insert(valve.id);
        }
    }
    res
}

#[derive(Debug)]
struct Valve {
    id: usize,
    name: String,
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
            name: valve[0].to_string(),
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
    'outer: while steps < 30 {
        steps += 1;
        for _i in 0..que.len() {
            let node = que.pop_front().unwrap();
            visited.insert(node);
            if node == target {
                break 'outer;
            }
            for neighbor in map.get(&node).unwrap().connected_to.clone() {
                if !visited.contains(&neighbor) {
                    que.push_back(neighbor);
                }
            }
        }
    }

    steps
}
const TESTINPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
