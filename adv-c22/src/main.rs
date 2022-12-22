use std::fs;
const H: usize = 12;
const W: usize = 20;
use nom::bytes::complete::take_until;
use nom::character::complete::{alpha0, digit0};
use nom::sequence::preceded;
use nom::{
    bytes::complete::{tag, take},
    *,
};

fn main() {
    let input = fs::read_to_string("./testinput.txt").unwrap();
    let mut map = [[Material::End; W]; H];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                ' ' => map[y][x] = Material::End,
                '.' => map[y][x] = Material::Tile,
                '#' => map[y][x] = Material::Wall,
                _ => (),
            }
        }
    }
    grid_printer(map);
    let (_, movelist) = parse_moves(MOVES).unwrap();
    //print!("");
    let mut currentpos = find_start(map);
    for moveop in movelist {
        match moveop {
            Move::Right => (),
            Move::Left => (),
            Move::Forward(steps) => (),
        }
    }
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Pos {
    x: usize,
    y: usize,
    dir: Dir,
}
impl Pos {
    fn move_steps(&mut self, map: [[Material; W]; H]) {
        let mut nextpos = self;
        match self.dir {
            Dir::Up => nextpos.y -= 1,
            Dir::Down => nextpos.y += 1,
            Dir::Left => nextpos.x -= 1,
            Dir::Right => nextpos.x += 1,
        }
        if map[nextpos.y][nextpos.x] == Material::End {
            match self.dir {
                Dir::Up => {
                    nextpos.y = H - 1;
                    while map[nextpos.y][nextpos.x] == Material::End {
                        nextpos.y -= 1;
                    }
                }
                Dir::Down => nextpos.y += 1,
                Dir::Left => nextpos.x -= 1,
                Dir::Right => nextpos.x += 1,
            }
        }
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
enum Material {
    End,
    Wall,
    Tile,
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Copy, Clone, PartialEq, Debug)]
enum Move {
    Right,
    Left,
    Forward(usize),
}
fn find_start(map: [[Material; W]; H]) -> Pos {
    let mut pos = Pos {
        x: 0,
        y: 0,
        dir: Dir::Right,
    };
    for (x, t) in map[0].iter().enumerate() {
        if *t == Material::Tile {
            pos.x = x;
            break;
        }
    }
    pos
}
fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let mut res = Vec::new();
    let mut parsing_input = input;
    loop {
        let (input, steps) = digit0(parsing_input)?;
        if steps.len() != 0 {
            res.push(Move::Forward(usize::from_str_radix(steps, 10).unwrap()));
        }
        let (input, dir) = alpha0(input)?;
        parsing_input = input;
        match dir {
            "R" => res.push(Move::Right),
            "L" => res.push(Move::Left),
            _ => (),
        }
        if parsing_input.len() == 0 {
            break;
        }
    }
    Ok((parsing_input, res))
}
fn grid_printer(grid: [[Material; W]; H]) {
    for row in grid {
        for seen in row {
            if seen == Material::Wall {
                print!("#");
            } else if seen == Material::Tile {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
const MOVES: &str = "10R5L5R10L4R5L5";
