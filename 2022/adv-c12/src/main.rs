use rayon::prelude::*;
use std::collections::VecDeque;
#[allow(unused_variables)]
use std::fs;
use std::slice::Iter;
use std::time::{Duration, Instant};
const H: usize = 41;
const W: usize = 136;
use std::thread;

fn main() {
    let builder = thread::Builder::new().stack_size(256 * 1024 * 1024);
    let handler = builder
        .spawn(|| {
            let input = fs::read_to_string("./input.txt").unwrap();
            let mut map: [[u8; W]; H] = [[0; W]; H];
            let mut end_pos = Pos { x: 0, y: 0 };
            let mut start_pos = vec![];
            let mut start_pos2 = Pos { x: 0, y: 0 };
            for (i, line) in input.lines().enumerate() {
                for (j, c) in line.chars().enumerate() {
                    //print!("{}", c);
                    match c {
                        'S' => {
                            map[i][j] = 'a' as u8 - 97;
                            start_pos2 = Pos { x: j, y: i };
                        }
                        'a' => {
                            map[i][j] = c as u8 - 97;
                            start_pos.push(Pos { x: j, y: i });
                        }
                        'E' => {
                            end_pos = Pos { x: j, y: i };
                            map[i][j] = 'z' as u8 - 97;
                        }
                        _ => map[i][j] = c as u8 - 97,
                    }
                }
            }

            let start_pt1 = Instant::now();
            let ans_pt1 = bfs(map, start_pos2, end_pos);
            let mut list_moves: Vec<u32> = vec![];
            //let dur_pt1 = start_pt1.elapsed();
            //let start_pt2 = Instant::now();
            println!("{}", start_pos.len());
            // start_pos.into_par_iter().for_each(|pos| {
            //     if let Some(steps) = bfs(map, pos, end_pos) {
            //         println!("{}", steps)
            //     }
            // });
            // let dur_pt2 = start_pt2.elapsed();
            //println!("time1 {:?} time2 {:?}", dur_pt1, dur_pt2);
            list_moves.sort();
            println!(
                "pt1: {} pt2: ",
                ans_pt1.unwrap(),
                //list_moves[0]
            );
        })
        .unwrap();

    handler.join().unwrap();

    // let ten_millis = Duration::from_millis(10000);

    // thread::sleep(ten_millis);
}
fn bfs(map: [[u8; W]; H], start: Pos, end: Pos) -> Option<usize> {
    let mut que = VecDeque::new();
    let mut visited_map = [[false; W]; H];
    let mut steps = 0;
    let mut ret = None;
    que.push_back(start);
    visited_map[start.y][start.x] = true;
    'outer: while !que.is_empty() {
        // grid_printer(visited_map);
        // let ten_millis = Duration::from_millis(500);
        // thread::sleep(ten_millis);
        steps += 1;
        for _ in 0..que.len() {
            let current_node = que.pop_front().unwrap();
            for next_move in valid_next_move(map, current_node) {
                if !visited_map[next_move.y][next_move.x] {
                    if end == next_move {
                        ret = Some(steps);
                        break 'outer;
                    } else {
                        visited_map[next_move.y][next_move.x] = true;
                        que.push_back(next_move);
                    }
                }
            }
        }
    }
    ret
}
fn valid_next_move(map: [[u8; W]; H], from: Pos) -> Vec<Pos> {
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
