use std::collections::{HashMap, HashSet};
fn main() {
    let elfmap = parse_map(TESTINPUT);
    find_next_moves(&elfmap);
    // calculate_next_moves
    //Do next moves
}
fn find_next_moves(elfmap: &HashMap<Pos, Elf>) -> HashMap<Pos, Elf> {
    let elfs = HashMap::new();
    for (pos, elf) in elfmap.mut_iter() {
        let mut dir = elf.lastdir;
        elf.lastdir = dir.get_next();
        for _ in 0..4 {
            dir = dir.get_next();
            if check_dir(elfmap, dir, &elf.currentpos){
                println!("Can move {} {} {:?}", elf.currentpos.x, elf.currentpos.y, dir);
            }

        }

    }
    elfs
}
fn check_dir(map: &HashMap<Pos, Elf>, dir: Dir, currentpos: &Pos) -> bool {
    let mut res = false;
    match dir {
        Dir::North => {
            let nw = &Pos {
                x: currentpos.x - 1,
                y: currentpos.y - 1,
            };
            let n = &Pos {
                x: currentpos.x,
                y: currentpos.y - 1,
            };
            let ne = &Pos {
                x: currentpos.x + 1,
                y: currentpos.y - 1,
            };
            if !map.contains_key(nw) && !map.contains_key(n) && !map.contains_key(ne) {
                res = true;
            }
        }
        Dir::South => {
            let sw = &Pos {
                x: currentpos.x - 1,
                y: currentpos.y + 1,
            };
            let s = &Pos {
                x: currentpos.x,
                y: currentpos.y + 1,
            };
            let se = &Pos {
                x: currentpos.x + 1,
                y: currentpos.y + 1,
            };
            if !map.contains_key(sw) && !map.contains_key(s) && !map.contains_key(se) {
                res = true;
            }
        }
        Dir::West => {
            let nw = &Pos {
                x: currentpos.x - 1,
                y: currentpos.y - 1,
            };
            let w = &Pos {
                x: currentpos.x - 1,
                y: currentpos.y,
            };
            let sw = &Pos {
                x: currentpos.x - 1,
                y: currentpos.y + 1,
            };
            if !map.contains_key(nw) && !map.contains_key(w) && !map.contains_key(sw) {
                res = true;
            }
        }
        Dir::East => {
            let ne = &Pos {
                x: currentpos.x + 1,
                y: currentpos.y - 1,
            };
            let e = &Pos {
                x: currentpos.x + 1,
                y: currentpos.y,
            };
            let se = &Pos {
                x: currentpos.x + 1,
                y: currentpos.y + 1,
            };
            if !map.contains_key(ne) && !map.contains_key(e) && !map.contains_key(se) {
                res = true;
            }
        }
    }
    res
}
fn parse_map(input: &str) -> HashMap<Pos, Elf> {
    let mut map = HashMap::new();
    let (mut max_x, mut max_y) = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if max_y < y {
                max_y = y;
            }
            if max_x < x {
                max_x = x;
            }
            match c {
                ' ' => {}
                '.' => {}
                '#' => {
                    map.insert(
                        Pos {
                            x: x as i32,
                            y: y as i32,
                        },
                        Elf {
                            currentpos: Pos {
                                x: x as i32,
                                y: y as i32,
                            },
                            newpos: Pos {
                                x: x as i32,
                                y: y as i32,
                            },
                            lastdir: Dir::East,
                        },
                    );
                }
                _ => {}
            }
        }
    }
    map
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Pos {
    x: i32,
    y: i32,
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Elf {
    currentpos: Pos,
    newpos: Pos,
    lastdir: Dir,
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
enum Dir {
    North,
    South,
    West,
    East,
}
impl Dir {
    fn get_next(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::East,
            Dir::East => Dir::North,
        }
    }
}
const TESTINPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
