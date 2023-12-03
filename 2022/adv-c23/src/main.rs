use rustc_hash::FxHashMap as HashMap;
use std::fs;
fn main() {
    let input = &fs::read_to_string("./input.txt").unwrap();
    let mut elfmap = parse_map(input);
    let mut dir = Dir::North;
    let mut move_num = 0;
    loop {
        move_num += 1;
        let nextmoves = find_next_moves(&elfmap, dir);
        if nextmoves.is_empty() {
            break;
        }
        for (_pos, nextmove) in nextmoves {
            elfmap.remove(&nextmove.currentpos);
            elfmap.insert(
                nextmove.newpos,
                Elf {
                    currentpos: nextmove.newpos,
                    newpos: nextmove.newpos,
                },
            );
        }
        //grid_printer(&elfmap);
        dir = dir.get_next();
        if move_num == 10 {
            calculate_aera(&elfmap);
        }
    }
    println!("pt2: {}", move_num);
}
fn calculate_aera(map: &HashMap<Pos, Elf>) -> usize {
    let (mut minx, mut maxx) = (0, 0);
    let (mut miny, mut maxy) = (0, 0);
    let mut number_of_elfs = 0;
    for (pos, _elf) in map {
        //println!("elf: x:{} y:{}", pos.x, pos.y);
        number_of_elfs += 1;
        if pos.x < minx {
            minx = pos.x
        }
        if pos.y < miny {
            miny = pos.y
        }
        if pos.x > maxx {
            maxx = pos.x
        }
        if pos.y > maxy {
            maxy = pos.y
        }
    }

    //println!("X from {} to {}", minx, maxx);
    //println!("Y from {} to {}", miny, maxy);
    let area = (1 + maxx - minx) * (1 + maxy - miny);
    println!("pt1: area: {}", area - number_of_elfs);
    0
}
fn grid_printer(map: &HashMap<Pos, Elf>) {
    for y in -2..10 {
        for x in -2..10 {
            if map.contains_key(&Pos { x, y }) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!();
}
fn find_next_moves(elfmap: &HashMap<Pos, Elf>, dir: Dir) -> HashMap<Pos, Elf> {
    let mut next_moves = HashMap::default();
    for (_pos, elf) in elfmap {
        let mut dir = dir;
        //elf.lastdir = dir.get_next();
        for _ in 0..4 {
            if check_dir(elfmap, dir, &elf.currentpos) {
                let mut elfmove = elf.clone();
                elfmove.newpos = elfmove.currentpos.step(dir);
                if next_moves.insert(elfmove.newpos, elfmove).is_some() {
                    next_moves.remove(&elfmove.newpos);
                }
                break;
            }
            dir = dir.get_next();
        }
    }
    next_moves
}
fn check_dir(map: &HashMap<Pos, Elf>, dir: Dir, currentpos: &Pos) -> bool {
    let mut res = false;
    let nw = map.contains_key(&Pos {
        x: currentpos.x - 1,
        y: currentpos.y - 1,
    });
    let n = map.contains_key(&Pos {
        x: currentpos.x,
        y: currentpos.y - 1,
    });
    let ne = map.contains_key(&Pos {
        x: currentpos.x + 1,
        y: currentpos.y - 1,
    });
    let sw = map.contains_key(&Pos {
        x: currentpos.x - 1,
        y: currentpos.y + 1,
    });
    let s = map.contains_key(&Pos {
        x: currentpos.x,
        y: currentpos.y + 1,
    });
    let se = map.contains_key(&Pos {
        x: currentpos.x + 1,
        y: currentpos.y + 1,
    });
    let e = map.contains_key(&Pos {
        x: currentpos.x + 1,
        y: currentpos.y,
    });
    let w = map.contains_key(&Pos {
        x: currentpos.x - 1,
        y: currentpos.y,
    });
    if !nw && !n && !ne && !w && !e && !sw && !s && !se {
        return false;
    }
    match dir {
        Dir::North => {
            if !nw && !n && !ne {
                res = true;
            }
        }
        Dir::South => {
            if !sw && !s && !se {
                res = true;
            }
        }
        Dir::West => {
            if !nw && !w && !sw {
                res = true;
            }
        }
        Dir::East => {
            if !ne && !e && !se {
                res = true;
            }
        }
    }
    res
}
fn parse_map(input: &str) -> HashMap<Pos, Elf> {
    let mut map = HashMap::default();
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
                        },
                    );
                }
                _ => {}
            }
        }
    }
    map
}
#[derive(Debug, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Eq)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn step(&self, dir: Dir) -> Pos {
        match dir {
            Dir::North => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Dir::South => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Dir::West => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Dir::East => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Elf {
    currentpos: Pos,
    newpos: Pos,
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn colision_detection() {
        let mut elfmap = HashMap::new();
        let elf1 = Elf {
            currentpos: Pos { x: 2, y: 0 },
            newpos: Pos { x: 0, y: 0 },
        };
        let elf2 = Elf {
            currentpos: Pos { x: 2, y: 2 },
            newpos: Pos { x: 0, y: 0 },
        };
        elfmap.insert(elf1.currentpos, elf1);
        elfmap.insert(elf2.currentpos, elf2);
        let next_moves = find_next_moves(&elfmap, Dir::North);
        assert!(next_moves.is_empty());
    }
}
