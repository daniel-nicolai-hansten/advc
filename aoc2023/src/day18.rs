use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::collections::VecDeque;
#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<(Dir, usize)> {
    let mut ret = vec![];
    for line in input.lines() {
        let splits: Vec<&str> = line.split_whitespace().collect();
        let dir = match splits[0] {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "D" => Dir::Down,
            "U" => Dir::Up,
            _ => Dir::None,
        };
        let dist = splits[1].parse::<usize>().unwrap();
        ret.push((dir, dist));
    }
    ret
}

#[aoc(day18, part1)]
fn part1(input: &[(Dir, usize)]) -> usize {
    let mut pos = IPos { x: 0, y: 0 };
    let mut visited = vec![];
    for (dir, dist) in input {
        for _ in 0..*dist {
            pos = match dir {
                Dir::Right => IPos {
                    y: pos.y,
                    x: pos.x + 1,
                },
                Dir::Left => IPos {
                    y: pos.y,
                    x: pos.x - 1,
                },
                Dir::Up => IPos {
                    y: pos.y - 1,
                    x: pos.x,
                },
                Dir::Down => IPos {
                    y: pos.y + 1,
                    x: pos.x,
                },
                Dir::None => pos,
            };
            visited.push(pos);
        }
    }
    let minx = visited.iter().min_by_key(|p|p.x).unwrap();
    let maxx = visited.iter().max_by_key(|p|p.x).unwrap();
    let miny = visited.iter().min_by_key(|p|p.y).unwrap();
    let maxy = visited.iter().max_by_key(|p|p.y).unwrap();
    println!("{}  {}  {}  {}", minx.x, maxx.x, miny.y, maxy.y);
    let xsize = minx.x.abs_diff(maxx.x) +10;
    let ysize = miny.y.abs_diff(maxy.y) +10;
    let mut map = vec![];
    let mut trench_tot = 0;
    for _ in 0..=ysize {
        let mut line = vec![];
        for _ in 0..=xsize {
            line.push(Ground::GroundLevel);
        }
        map.push(line);
    }
    for p in &visited {
        let x = p.x + minx.x.abs() +2;
        let x = x as usize;
        let y = p.y + miny.y.abs() +2;
        let y = y as usize;
        map[y][x] = Ground::Trench;
        trench_tot += 1;
    }

    let flood = flood(&map, &[Pos{x:0, y:0}, Pos{x: xsize, y: 0} , Pos{x: xsize, y: ysize}, Pos{x: 0, y: ysize}]);
    for y in 0..map.len() {
        for x in 0..map[0].len()  {
            if  map[y][x] == Ground::GroundLevel && !flood.contains(&Pos{x, y}) {
                map[y][x] = Ground::Trench;
                trench_tot += 1;
            }
        }
    }
    map_printer(&map);
    trench_tot
}

fn map_printer(map: &[Vec<Ground>]) {
    for line in map {
        for p in line {
            let c = match *p {
                Ground::GroundLevel => ".",
                Ground::Trench => "#",
            };
            print!("{c}");
        }
        println!();
    }
}
fn flood(map: &[Vec<Ground>], start: &[Pos]) -> HashSet<Pos> {
    let mut que = VecDeque::new();
    let mut visited_map = HashSet::new();
    for p in start {
        que.push_back(*p);
        visited_map.insert(*p);
    }
    
    while !que.is_empty() {
        for _ in 0..que.len() {
            let current_node = que.pop_front().unwrap();
            for next_move in valid_next_move_hr(map, current_node) {
                if !visited_map.contains(&next_move) {
                    visited_map.insert(next_move);
                    que.push_back(next_move);
                }
            }
        }
    }
    visited_map
}
fn valid_next_move_hr(map: &[Vec<Ground>], from: Pos) -> Vec<Pos> {
    let mut results = vec![];
    let (h, w) = (map.len(), map[0].len());
    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right]{
        let mut next = from;
        match dir {
            Dir::Up => {
                if from.y > 0 {
                    next.y -= 1;
                }
            }
            Dir::Down => {
                if from.y < h - 1 {
                    next.y += 1;
                }
            }
            Dir::Left => {
                if from.x > 0 {
                    next.x -= 1;
                }
            }
            Dir::Right => {
                if from.x < w - 1 {
                    next.x += 1;
                }
            }
            Dir::None => (),
        }
        if next != from && map[next.y][next.x] == Ground::GroundLevel {
            results.push(next);
        }
    }
    results
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Ground {
    Trench,
    GroundLevel,
}
#[aoc(day18, part2)]
fn part2(input: &[(Dir, usize)]) -> String {
    todo!()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
    None,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct IPos {
    x: isize,
    y: isize,
}
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 62);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
