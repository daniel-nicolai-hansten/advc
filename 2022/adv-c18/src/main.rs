use std::collections::{HashMap, HashSet, VecDeque};
const H: usize = 500000;
const W: usize = 7;
#[allow(unused_variables)]
use std::fs;
const X: usize=20;
const Y: usize=20;
const Z: usize=20;
fn main() {
    let input = &fs::read_to_string("./input.txt").unwrap();
    let mut lava = parse_input(input);
    let mut surface = 0;
    for splash in &lava {
        let currentsurface = check_for_neighbor(splash, &lava);
        surface += currentsurface;
        println!("neighbors {}", currentsurface);
    }
    println!("surface {}", surface);
    
    for x in 0..20 {
        for y in 0..20 {
            for z in 0..20 {
                let checkblock = Pos{x: x, y:y, z:z};
                if find_airpocket(&checkblock, &lava){
                    lava.insert(checkblock);
                }   
            }
        }
    }
    surface = 0;
    for splash in &lava {
        let currentsurface = check_for_neighbor(splash, &lava);
        surface += currentsurface;
        //println!("neighbors {}", currentsurface);
    }
    println!("surface2 {}", surface);
}




fn parse_input(input: &str) -> HashSet<Pos>{
    //let mut grid3d = [[[0; Z];X];Y];
    let mut lava = HashSet::new();
    for line in input.lines() {
        let mut splits = line.split(',');
            let (x, y, z) = (splits.next().unwrap(), splits.next().unwrap(), splits.next().unwrap());
            println!("X: {} y, {}, z {}", x, y, z,);
            lava.insert(Pos {x: x.parse().unwrap(), y: y.parse().unwrap(), z: z.parse().unwrap()});
        }
    lava
    }



fn check_for_neighbor(block: &Pos, lava: &HashSet<Pos>) -> usize{
    let mut ret = 0;
    if !lava.contains( &Pos{x: block.x +1, y: block.y, z: block.z}){
        ret +=1;
    }
    if !lava.contains( &Pos{x: block.x -1, y: block.y, z: block.z}){
        ret +=1;
    }
    if !lava.contains( &Pos{x: block.x, y: block.y +1, z: block.z}){
        ret +=1;
    }
    if !lava.contains( &Pos{x: block.x, y: block.y -1, z: block.z}){
        ret +=1;
    }
    if !lava.contains( &Pos{x: block.x, y: block.y, z: block.z +1 }){
        ret +=1;
    }
    if !lava.contains( &Pos{x: block.x, y: block.y, z: block.z -1}){
        ret +=1;
    }

ret
}
fn find_airpocket(block: &Pos, lava: &HashSet<Pos>) -> bool{
let is_potential_airpocket = check_all_dirs(block, lava);
let mut airpocket = false;
let mut checked = HashSet::new();
let mut bfsque = VecDeque::new();
bfsque.push_back(block.clone());
'bfs: loop {
    if let Some(currentpos) = bfsque.pop_front() {
        if check_all_dirs(&currentpos, lava) {
            for nextmove in find_next_bfsmove(&currentpos, lava){
                if !checked.contains(&nextmove){
                    checked.insert(nextmove.clone());
                    bfsque.push_back(nextmove.clone());
                }
            }
        } else {
            airpocket = false;
            break 'bfs;
        }
    } else {
        airpocket = true;
        break 'bfs;
    }
}

airpocket
}

fn find_next_bfsmove(block: &Pos, lava: &HashSet<Pos>) -> Vec<Pos>{
    let mut ret = vec![];
    if !lava.contains( &Pos{x: block.x +1, y: block.y, z: block.z}) && block.x +1 < 25 {
       ret.push(Pos{x: block.x +1, y: block.y, z: block.z});
    }
    if !lava.contains( &Pos{x: block.x -1, y: block.y, z: block.z}) && block.x -1 > -2 {
        ret.push(Pos{x: block.x -1, y: block.y, z: block.z});
    }
    if !lava.contains( &Pos{x: block.x, y: block.y +1, z: block.z})&& block.y +1 < 25{
        ret.push(Pos{x: block.x , y: block.y +1, z: block.z});
    }
    if !lava.contains( &Pos{x: block.x, y: block.y -1, z: block.z})&& block.x -1 > -2 {
        ret.push(Pos{x: block.x, y: block.y -1, z: block.z});
    }
    if !lava.contains( &Pos{x: block.x, y: block.y, z: block.z +1 })&& block.z +1 < 25{
        ret.push(Pos{x: block.x, y: block.y, z: block.z +1});
    }
    if !lava.contains( &Pos{x: block.x, y: block.y, z: block.z -1})&& block.x -1 > -2 {
        ret.push(Pos{x: block.x, y: block.y, z: block.z -1});
    }
ret
}

fn check_all_dirs(block: &Pos, lava: &HashSet<Pos>) -> bool{
let mut ret = false;
let mut neighbor = 0;

    if lava.contains(&block) {
        return false;
}
'inner: for i in 0..20 {
    if lava.contains( &Pos{x: block.x + i, y: block.y, z: block.z}){
        neighbor +=1;
        break 'inner;
    }
}
'inner: for i in 0..20 {
    if lava.contains( &Pos{x: block.x - i, y: block.y, z: block.z}){
        neighbor +=1;
        break 'inner;
    }
}
'inner: for i in 0..20 {
    if lava.contains( &Pos{x: block.x, y: block.y +i , z: block.z}){
        neighbor +=1;
        break 'inner;
    }
}
'inner: for i in 0..20 {
    if lava.contains( &Pos{x: block.x, y: block.y -i, z: block.z}){
        neighbor +=1;
        break 'inner;
    }
}
'inner: for i in 0..20 {
    if lava.contains( &Pos{x: block.x, y: block.y, z: block.z +i}){
        neighbor +=1;
        break 'inner;
    }
}
'inner: for i in 0..20 {
    if lava.contains( &Pos{x: block.x, y: block.y, z: block.z -i}){
        neighbor +=1;
        break 'inner;
    }
}
if neighbor == 6 {
    //println!("Found air at {:?}", block);
    ret = true;
}
ret
}
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
}
const TESTINPUT: &str = 
"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";