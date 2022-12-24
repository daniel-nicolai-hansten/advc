const H: usize = 27;
const W: usize = 122;
use std::collections::VecDeque;
use itertools::Itertools;
use std::fs;
fn main() {
    let input = &fs::read_to_string("./input.txt").unwrap();
    let (map, mut blizzards) = parse_input(input);
    let mut workque = VecDeque::new();
    let mut steps = 0;
    workque.push_back(Pos { x: 1, y: 0 });
    'bfs: loop {
        let blizzard_map = draw_blizzard(&map, &mut blizzards);
        grid_printer(&blizzard_map);
        if workque.len() == 0 {
            panic!();
        } else if workque.len() > H * W {
            let tmp_workque = workque.clone();
            let tmp_que: Vec<&Pos> = tmp_workque.iter().unique().collect();
            workque.clear();
            for item in tmp_que {
                workque.push_back(*item);
            }
        }
        for _ in 0..workque.len() {
            let pos = workque.pop_front().unwrap();
            if pos.y == H - 1 {
                break 'bfs;
            }
            for nextpos in find_next_valid_move(&blizzard_map, pos) {
                workque.push_back(nextpos);
            }
        }
        steps += 1;
    }
    println!("Steps: {}", steps);
    workque.clear();
    workque.push_back(Pos { x: W -2, y: H -1 });
    'bfs: loop {
        let blizzard_map = draw_blizzard(&map, &mut blizzards);
        grid_printer(&blizzard_map);
        if workque.len() == 0 {
            panic!();
        } else if workque.len() > H * W {
            let tmp_workque = workque.clone();
            let tmp_que: Vec<&Pos> = tmp_workque.iter().unique().collect();
            workque.clear();
            for item in tmp_que {
                workque.push_back(*item);
            }
        }
        for _ in 0..workque.len() {
            let pos = workque.pop_front().unwrap();
            if pos.y == 0 {
                break 'bfs;
            }
            for nextpos in find_next_valid_move(&blizzard_map, pos) {
                workque.push_back(nextpos);
            }
        }
        steps += 1;
    }
    println!("Steps: {}", steps);
    workque.clear();
    workque.push_back(Pos { x: 1, y: 0 });
    'bfs: loop {
        let blizzard_map = draw_blizzard(&map, &mut blizzards);
        grid_printer(&blizzard_map);
        if workque.len() == 0 {
            panic!();
        } else if workque.len() > H * W {
            let tmp_workque = workque.clone();
            let tmp_que: Vec<&Pos> = tmp_workque.iter().unique().collect();
            workque.clear();
            for item in tmp_que {
                workque.push_back(*item);
            }
        }
        for _ in 0..workque.len() {
            let pos = workque.pop_front().unwrap();
            if pos.y == H - 1 {
                break 'bfs;
            }
            for nextpos in find_next_valid_move(&blizzard_map, pos) {
                workque.push_back(nextpos);
            }
        }
        steps += 1;
    }
    println!("Steps: {}", steps);
}
fn grid_printer(map: &[[Material; W]; H]) {
    for row in map {
        for point in row {
            match point {
                Material::Open => print!("."),
                Material::Wall => print!("#"),
                Material::Blizzard(dir) => match dir {
                    BlizzardTypes::Up => print!("^"),
                    BlizzardTypes::Down => print!("v"),
                    BlizzardTypes::Left => print!("<"),
                    BlizzardTypes::Right =>  print!(">"),
            },
            }
        }
        println!("");
    }

}
fn find_next_valid_move(map: &[[Material; W]; H], pos: Pos) -> Vec<Pos> {
    let mut valid_moves = Vec::new();
    if map[pos.y][pos.x] == Material::Open {
        // Wait
        valid_moves.push(Pos { x: pos.x, y: pos.y })
    }
    if map[pos.y][pos.x +1] == Material::Open {
        // Right
        valid_moves.push(Pos {
            x: pos.x + 1,
            y: pos.y,
        })
    }

    if map[pos.y][pos.x -1] == Material::Open {
        // Left
        valid_moves.push(Pos {
            x: pos.x - 1,
            y: pos.y,
        })
    }

    if pos.y != H -1 && map[pos.y +1][pos.x] == Material::Open {
        // Down
        valid_moves.push(Pos {
            x: pos.x,
            y: pos.y +1,
        })
    }

    if pos.y != 0 && map[pos.y -1][pos.x] == Material::Open {
        // Up
        valid_moves.push(Pos {
            x: pos.x,
            y: pos.y - 1,
        })
    }
    valid_moves
}
fn parse_input(input: &str) -> ([[Material; W]; H], Vec<Blizzard>) {
    let mut blizzards: Vec<Blizzard> = Vec::new();
    let mut map = [[Material::Open; W]; H];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' | '<' | 'v' | '^' => blizzards.push(Blizzard {
                    pos: Pos { x, y },
                    dir: match c {
                        '>' => BlizzardTypes::Right,
                        '<' => BlizzardTypes::Left,
                        'v' => BlizzardTypes::Down,
                        '^' => BlizzardTypes::Up,
                        _ => panic!(),
                    },
                }),
                '#' => map[y][x] = Material::Wall,
                '.' => (),
                _ => panic!(),
            }
        }
    }
    (map, blizzards)
}
fn draw_blizzard(map: &[[Material; W]; H], blizzards: &mut Vec<Blizzard>) -> [[Material; W]; H] {
    let mut wip_map = map.clone();
    for blizzard in blizzards.iter_mut() {
        match blizzard.dir {
            BlizzardTypes::Left => {
                blizzard.pos.x -= 1;
                if blizzard.pos.x == 0 {
                    blizzard.pos.x = W - 2;
                }
            }
            BlizzardTypes::Right => {
                blizzard.pos.x += 1;
                if blizzard.pos.x == W - 1 {
                    blizzard.pos.x = 1;
                }
            }
            BlizzardTypes::Up => {
                blizzard.pos.y -= 1;
                if blizzard.pos.y == 0 {
                    blizzard.pos.y = H - 2;
                }
            }
            BlizzardTypes::Down => {
                blizzard.pos.y += 1;
                if blizzard.pos.y == H - 1 {
                    blizzard.pos.y = 1;
                }
            }
        }
        wip_map[blizzard.pos.y][blizzard.pos.x] = Material::Blizzard(blizzard.dir);
    }
    wip_map
}
#[derive(Copy, Clone, PartialEq)]
enum Material {
    Wall,
    Blizzard(BlizzardTypes),
    Open,
}
#[derive(Copy, Clone, PartialEq)]
enum BlizzardTypes {
    Left,
    Right,
    Up,
    Down,
}
struct Blizzard {
    dir: BlizzardTypes,
    pos: Pos,
}
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}
const TESTINPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
