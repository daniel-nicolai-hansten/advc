#[allow(unused_variables)]
use std::fs;
const H: usize = 188;
const W: usize = 500;
const OFFSETX: usize = 250;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut cave = Cave::new();
    let mut floor = 0;
    for line in input.lines() {
        let mut currentrock = vec![];
        let cordinates: Vec<&str> = line.split(" -> ").collect();
        for cordinate in cordinates {
            let split_cordinate: Vec<&str> = cordinate.split(",").collect();
            let pos = Pos {
                x: split_cordinate[0].parse::<usize>().unwrap() - OFFSETX,
                y: split_cordinate[1].parse::<usize>().unwrap(),
            };
            if pos.y + 2 > floor {
                floor = pos.y + 2;
            }
            currentrock.push(pos);
        }
        cave.draw_rock(currentrock);
    }
    let mut total_sands = 1;
    while cave.model_sand().is_some() {
        total_sands += 1;
    }
    let pt1_res = total_sands - 1;
    let cave_floor = vec![Pos { y: floor, x: 0 }, Pos { y: floor, x: W - 1 }];
    cave.draw_rock(cave_floor);
    while cave.model_sand().is_some() {
        total_sands += 1;
    }

    //cave.grid_printer();
    println!(
        "Total sands: pt1: {} pt2: {} Floor {}",
        pt1_res, total_sands, floor
    );
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Pos {
    x: usize,
    y: usize,
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
enum CaveMaterial {
    Air,
    Sand,
    Rock,
}
struct Cave {
    map: [[CaveMaterial; W]; H],
}
impl Cave {
    fn new() -> Cave {
        Cave {
            map: [[CaveMaterial::Air; W]; H],
        }
    }
    fn draw_rock(&mut self, rock: Vec<Pos>) {
        let mut last_pos: Option<Pos> = None;
        for edge in rock {
            if let Some(lpos) = last_pos {
                if lpos.x < edge.x {
                    for x in lpos.x..=edge.x {
                        self.map[edge.y][x] = CaveMaterial::Rock;
                    }
                }
                if lpos.x > edge.x {
                    for x in edge.x..=lpos.x {
                        self.map[edge.y][x] = CaveMaterial::Rock;
                    }
                }
                if lpos.y < edge.y {
                    for y in lpos.y..=edge.y {
                        self.map[y][edge.x] = CaveMaterial::Rock;
                    }
                }
                if lpos.y > edge.y {
                    for y in edge.y..=lpos.y {
                        self.map[y][edge.x] = CaveMaterial::Rock;
                    }
                }
            }
            last_pos = Some(edge);
        }
    }
    #[allow(dead_code)]
    fn grid_printer(&self) {
        for row in self.map {
            for material in row {
                match material {
                    CaveMaterial::Air => print!("."),
                    CaveMaterial::Rock => print!("#"),
                    CaveMaterial::Sand => print!("o"),
                }
            }
            println!("");
        }
    }
    fn model_sand(&mut self) -> Option<()> {
        let mut res = None;
        let mut sand_pos = SAND_START;
        loop {
            if sand_pos.y + 1 == H {
                break;
            } else if self.map[sand_pos.y + 1][sand_pos.x] == CaveMaterial::Air {
                sand_pos.y += 1;
            } else if self.map[sand_pos.y + 1][sand_pos.x - 1] == CaveMaterial::Air {
                sand_pos.y += 1;
                sand_pos.x -= 1;
            } else if self.map[sand_pos.y + 1][sand_pos.x + 1] == CaveMaterial::Air {
                sand_pos.y += 1;
                sand_pos.x += 1;
            } else {
                res = Some(());
                self.map[sand_pos.y][sand_pos.x] = CaveMaterial::Sand;
                break;
            }
        }
        if sand_pos.y != 0 {
            res
        } else {
            None
        }
    }
}
const SAND_START: Pos = Pos {
    x: 500 - OFFSETX,
    y: 0,
};
