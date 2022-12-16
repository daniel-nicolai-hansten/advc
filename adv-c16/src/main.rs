use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_variables)]
use std::fs;
use std::process::exit;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let valves_map = parse_input(&input);
    //println!("{:?}", valves_map);
    let mut working_valves = find_working_valves(&valves_map);
    let (mut path_you, mut path_elephant) = (vec![], vec![]);
    let (mut time_left_you, mut time_left_elephant) = (26, 26);
    let (mut pos_you, mut pos_elephant) = (
        usize::from_str_radix("AA", 36).unwrap(),
        usize::from_str_radix("AA", 36).unwrap(),
    );
    while time_left_you > 0 || time_left_elephant > 0 {
        let (_, next_move_you, next_move_elephant) = find_next_best_move(
            &valves_map,
            (pos_you, pos_elephant),
            (time_left_you, time_left_elephant),
            working_valves.clone().into_iter().collect(),
        );
        path_you.push(next_move_you);
        path_elephant.push(next_move_elephant);
        let next_move_steps_you = bfs_calculate_steps(&valves_map, pos_you, next_move_you);
        let next_move_steps_elephant =
            bfs_calculate_steps(&valves_map, pos_elephant, next_move_elephant);
        if !(time_left_you < next_move_steps_you) && next_move_you != 0 {
            time_left_you -= next_move_steps_you;
            pos_you = next_move_you;
            working_valves.remove(&next_move_you);
            println!(
                "You move: {} steps {}",
                valves_map.get(&next_move_you).unwrap().name,
                next_move_steps_you
            );
        } else {
            time_left_you = 0;
        }
        if !(time_left_elephant < next_move_steps_elephant) && next_move_elephant != 0 {
            pos_elephant = next_move_elephant;
            working_valves.remove(&next_move_elephant);
            println!(
                "Elephant move: {} steps {}",
                valves_map.get(&next_move_elephant).unwrap().name,
                next_move_steps_elephant
            );
        } else {
            time_left_elephant = 0;
        }
    }

    println!(
        "total flow for path {}   {:?}",
        calculate_flow(
            &path_you,
            usize::from_str_radix("AA", 36).unwrap(),
            &valves_map,
            26
        ) + calculate_flow(
            &path_elephant,
            usize::from_str_radix("AA", 36).unwrap(),
            &valves_map,
            26
        ),
        path_you
    );
}

fn calculate_flow3(
    valvelist: &Vec<&&usize>,
    startpos: usize,
    map: &HashMap<usize, Valve>,
    time_left: usize,
) -> usize {
    let mut time = time_left;
    let mut flow = 0;
    let mut pos = startpos;
    for valve in valvelist {
        let traveltime = get_steps(map, pos, ***valve);
        if time < traveltime {
            break;
        }
        time -= traveltime;
        pos = ***valve;
        flow += time * map.get(&valve).unwrap().flow_rate;
    }

    flow
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
        let traveltime = get_steps(map, pos, **valve);
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
        let traveltime = get_steps(map, pos, *valve);
        if time < traveltime {
            break;
        }
        time -= traveltime;
        pos = *valve;
        println!("{:?}", valvelist);
        if let Some(vlv) = map.get(&valve) {
            flow += time * vlv.flow_rate;
        }
    }

    flow
}
fn find_next_best_move(
    map: &HashMap<usize, Valve>,
    pos: (usize, usize),
    time_left: (usize, usize),
    valves_left: Vec<usize>,
) -> (usize, usize, usize) {
    let mut res = (0, 0, 0);
    let (pos_you, pos_elephant) = pos;
    let (time_left_you, time_left_elephant) = time_left;
    if valves_left.len() > 1 {
        //let compute_result = valves_left.clone().into_par_iter().map(|valve| {
        // let local_valves_left: Vec<usize> = valves_left
        //     .clone()
        //     .into_iter()
        //     .filter(|&x| x != valve)
        //     .collect();
        let mut sulotions: Vec<(usize, usize, usize)> = vec![];
        let mut permut = 5;
        if valves_left.len() <= permut * 2 {
            permut = valves_left.len() / 2;
        }
        for path in valves_left.iter().permutations(permut) {
            //print!(".");
            let flow1 = calculate_flow2(&path, pos_you, map, time_left_you);
            let mut flow2 = 0;
            let valves_left_for_elephant: Vec<&usize> =
                valves_left.iter().filter(|x| !path.contains(x)).collect();
            //println!("pathlists: {:?}  {:?}", path, valves_left_for_elephant);
            let mut permut2 = permut;
            let mut elepant_path = 0;
            if valves_left_for_elephant.len() <= permut2 {
                permut2 = valves_left_for_elephant.len();
            }

            for path2 in valves_left_for_elephant
                .clone()
                .iter()
                .permutations(permut2)
            {
                let flow_t = calculate_flow3(&path2, pos_elephant, map, time_left_elephant);
                if flow_t > flow2 {
                    flow2 = flow_t;
                    elepant_path = **path2[0];
                }
            }
            let mut you_path = 0;
            if path.len() != 0 {
                you_path = *path[0];
            }
            sulotions.push((flow1 + flow2, you_path, elepant_path));
            //println!("{:?}, {}", path, elepant_path);
        }
        sulotions.sort();
        //println!("{:?}", sulotions);
        res = sulotions.pop().unwrap()
        //});
    }

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

#[derive(Debug, Clone)]
struct Valve {
    id: usize,
    name: String,
    flow_rate: usize,
    connected_to: Vec<usize>,
    pathcost: [usize; 15],
}
impl Valve {
    fn calculcate_pathcost(&self, map: &HashMap<usize, Valve>) -> Valve {
        let mut ret = self.clone();
        for valve in VALVES_WORKING {
            let pathcost = bfs_calculate_steps(map, self.id, valve);
            ret.pathcost[get_valveid(valve)] = pathcost;
        }

        ret
    }
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
            pathcost: [0; 15],
        };
        valves.insert(current_valve.id, current_valve);
    }
    let mut valves2 = HashMap::new();
    for (id, valve) in &valves {
        valves2.insert(*id, valve.calculcate_pathcost(&valves));
    }
    valves2
}
fn get_steps(map: &HashMap<usize, Valve>, start: usize, target: usize) -> usize {
    let valve = map.get(&start);

    0
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

const JT: usize = 713;
const PH: usize = 917;
const IR: usize = 675;
const SV: usize = 1039;
const UV: usize = 1111;
const EZ: usize = 539;
const KE: usize = 734;
const OY: usize = 898;
const NN: usize = 851;
const FU: usize = 570;
const PT: usize = 929;
const IF: usize = 663;
const TO: usize = 1068;
const FC: usize = 552;
const QG: usize = 952;
const VALVES_WORKING: [usize; 15] = [JT, PH, IR, SV, UV, EZ, KE, OY, NN, FU, PT, IF, TO, FC, QG];

fn get_valveid(target: usize) -> usize {
    let index = match target {
        JT => 0,
        PH => 1,
        IR => 2,
        SV => 3,
        UV => 4,
        EZ => 5,
        KE => 6,
        OY => 7,
        NN => 8,
        FU => 9,
        PT => 10,
        IF => 11,
        TO => 12,
        FC => 13,
        QG => 14,
        _ => 99,
    };
    index
}
