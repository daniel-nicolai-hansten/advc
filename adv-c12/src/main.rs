use std::collections::VecDeque;
use std::collections::HashSet;
#[allow(unused_variables)]
use std::fs;
use std::slice::Iter;
const H: usize = 41;
const W: usize = 136;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut map = [[0; W]; H];
    let mut end_pos = Pos { x: 0, y: 0 };
    let mut start_pos = vec![];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            //print!("{}", c);
            match c {
                'S' => {
                    map[i][j] = 'a' as usize - 97;
                    start_pos.push(Pos { x: j, y: i });
                }
                'a' => {
                    map[i][j] = c as usize - 97;
                    start_pos.push(Pos { x: j, y: i });
                }
                'E' => {
                    end_pos = Pos { x: j, y: i };
                    map[i][j] = 'z' as usize - 97;
                }
                _ => map[i][j] = c as usize - 97,
            }
        }
    }
    let mut list_moves = vec![];
    println!("{:?}", start_pos);
    for pos in start_pos {
        if let Some(steps_taken) = bfs(map, pos, end_pos){
            println!("y: {}  x: {} steps: {}", pos.y, pos.x, steps_taken);
            list_moves.push(bfs(map, pos, end_pos));
        }
        
    }
    list_moves.sort();
    println!("{:?}", list_moves);
}
fn bfs(map: [[usize; W]; H], start: Pos, end: Pos) -> Option<usize> {
    let mut que = VecDeque::new();
    let mut visited = HashSet::new();
    let mut visited_map = [[false; W]; H];
    let mut steps = 0;
    let mut ret = None;
    que.push_back(start);
    visited.insert(start);
    'outer: while !que.is_empty() {
        steps += 1;
        //println!("Step {}", steps);
        //grid_printer(visited_map);
        for _ in 0..que.len() {
            let current_node = que.pop_front().unwrap();
            //visited.insert(current_node);
            for next_move in valid_next_move(map, current_node) {
                if !visited_map[next_move.y][next_move.x] {
                    if end == next_move {
                        ret = Some(steps);
                        break 'outer;
                    } else {
                    visited_map[next_move.y][next_move.x] = true;
                    que.push_back(next_move);}
                }
            }
        }
        //     for v in graph[current_node as usize].iter() {
        //         if *v == end_node {
        //             prev[*v as usize] = Some(current_node);
        //             break 'outer;
        //         }

        //         if !visisted_vertices[*v as usize] {
        //             que.enqueue(*v);
        //             visisted_vertices[*v as usize] = true;
        //             prev[*v as usize] = Some(current_node);
        //         }
        //     }
    }
    ret
}
fn valid_next_move(map: [[usize; W]; H], from: Pos) -> Vec<Pos> {
    let mut results = vec![];
    for dir in Dir::iterator() {
        let mut next = from;
        match dir {
            Dir::Up => {
                if from.y > 0 {
                    next.y -= 1;
                }
            }
            Dir::Down => {
                if from.y < H - 1 {
                    next.y += 1;
                }
            }
            Dir::Left => {
                if from.x > 0 {
                    next.x -= 1;
                }
            }
            Dir::Right => {
                if from.x < W - 1 {
                    next.x += 1;
                }
            }
        }
        if next != from && (map[from.y][from.x] + 1) >= map[next.y][next.x] {
            results.push(next);
        }
    }
    results
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    pub fn iterator() -> Iter<'static, Dir> {
        static DIRECTIONS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        DIRECTIONS.iter()
    }
}
const TESTINPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

fn grid_printer(grid: [[bool; W]; H]) {
    for row in grid {
        for seen in row {
            if seen {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}