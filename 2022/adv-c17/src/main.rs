//use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
const H: usize = 5000;
const W: usize = 7;
#[allow(unused_variables)]
use std::fs;

fn main() {
    let input = &fs::read_to_string("./input.txt").unwrap();
    let moves = parse_input(input);
    let mut grid = [[false; W]; H];
    let mut rocks = Rocks::new();
    let mut top_rock = Pos { y: H, x: W };

    let mut total_rocks:u64 = 0;
    let mut movesit = moves.iter();
    'rock: loop {
        total_rocks += 1;
        let mut rock = rocks.get_next();
        rock.move_to_start(&top_rock);

        'gas: loop {
            if let Some(dir) = movesit.next() {
                rock.move_x(&grid, dir);
                //println!("X move: {:?}, dir: {:?}", rock, dir);
                if rock.move_y(&grid).is_none() {
                    break 'gas;
                }
                //println!("y move: {:?}", rock);
            } else {
                movesit = moves.iter();
            }
        }
        let top_lastrock = rock.place(&mut grid);
        if top_lastrock < top_rock.y {
            top_rock.y = top_lastrock;
        }
        
        if total_rocks >= 2023{
            break 'rock;
        }
    }
    //grid_printer(grid);
    // println!("secs:: {}", (1000000000000 as u64/ 500000) /60/60/24);
    println!("Toppos: {} ", H - top_rock.y);
}

#[derive(Debug)]
enum GasDir {
    Left,
    Right,
}

struct InfiniteGrid {
    grid: [[bool;7 ]; 64]
    head: u64,
    tail: u64,

}
impl InfiniteGrid {
    fn new() -> InfiniteGrid {
        InfiniteGrid { grid: [[false;7];64], head: 0, tail: 63 }
    }
    fn prune(&mut self) {
        
    }

}
#[derive(Clone, Debug)]
struct Rock {
    shape: Vec<Pos>,
}
impl Rock {
    fn move_x(&mut self, grid: &[[bool; W]; H], dir: &GasDir) {
        let mut new_shape = self.shape.clone();
        let mut canmove = true;
        match dir {
            GasDir::Right => {
                for (i, pos) in self.shape.iter().enumerate() {
                    new_shape[i] = pos.clone();
                    new_shape[i].x += 1;
                    if new_shape[i].x >= W || grid[new_shape[i].y][new_shape[i].x] {
                        canmove = false;
                        break;
                    }
                }
            }
            GasDir::Left => {
                for (i, pos) in self.shape.iter().enumerate() {
                    if pos.x != 0 {
                        new_shape[i] = pos.clone();
                        new_shape[i].x -= 1;
                    } else {
                        canmove = false;
                        break;
                    }
                    if grid[new_shape[i].y][new_shape[i].x] {
                        canmove = false;
                        break;
                    }
                }
            }
        }
        if canmove {
            self.shape = new_shape;
        }
    }
    fn move_y(&mut self, grid: &[[bool; W]; H]) -> Option<()> {
        let mut new_shape = self.shape.clone();
        let mut canmove = true;
        let mut ret = None;
        for (i, pos) in self.shape.iter().enumerate() {
            new_shape[i].y += 1;
            //println!("newshape: {:?} i: {}", new_shape, i);
            if new_shape[i].y >= H || grid[new_shape[i].y][new_shape[i].x] {
                canmove = false;
                break;
            }
        }
        if canmove {
            ret = Some(());
            self.shape = new_shape;
        }
        ret
    }
    fn move_to_start(&mut self, lastrock: &Pos) {
        let mut new_shape = self.shape.clone();
        let mut startpos = 0;
        for pos in &self.shape {
            if pos.y > startpos {
                startpos = pos.y;
            }
        }

        for (i, pos) in self.shape.iter().enumerate() {
            new_shape[i].y += lastrock.y - startpos - 4;
            new_shape[i].x += 2;
        }
        self.shape = new_shape;
    }
    fn place(&self, grid: &mut [[bool; W]; H]) -> usize {
        let mut toppos = H;
        for pos in &self.shape {
            grid[pos.y][pos.x] = true;
            if toppos > pos.y {
                toppos = pos.y;
                //println!("toppos {}", toppos);
            }
        }
        toppos
    }
}
#[derive(Clone, Debug)]
struct Rocks {
    lasttype: usize,
    shapes: Vec<Rock>,
}
impl Rocks {
    fn new() -> Rocks {
        Rocks {
            lasttype: 4,
            shapes: parse_rocks(ROCKS),
        }
    }

    fn get_next(&mut self) -> Rock {
        self.lasttype += 1;
        if self.lasttype >= 5 {
            self.lasttype = 0;
        }
        self.shapes[self.lasttype].clone()
    }
}
#[derive(Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> Vec<GasDir> {
    let mut ret = vec![];
    for line in input.lines() {
        for c in line.chars() {
            match c {
                '>' => ret.push(GasDir::Right),
                '<' => ret.push(GasDir::Left),
                _ => (),
            }
        }
    }
    ret
}
fn parse_rocks(rocklist: [&str; 5]) -> Vec<Rock> {
    let mut ret = vec![];
    for rock in rocklist {
        let mut ret_rock = vec![];
        let mut y = 0;
        for line in rock.lines() {
            print!("{}", line);
            let mut x = 0;
            for c in line.chars() {
                match c {
                    '.' => x += 1,
                    '#' => {
                        let pos = Pos { x, y };
                        ret_rock.push(pos);
                        x += 1;
                    }
                    _ => (),
                }
            }
            y += 1;
        }
        ret.push(Rock { shape: ret_rock });
        println!("");
    }
    ret
}
const TESTINPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
const ROCKS: [&str; 5] = [
    "####",
    ".#.
###
.#.",
    "..#
..#
###",
    "#
#
#
#",
    "##
##",
];

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
