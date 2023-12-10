use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let (map, start) = parse_input(input);
    let steps = bfs(&map, start);
    println!("{steps:?}");
}
fn parse_input(input: &str) -> (Vec<Vec<char>>, Pos) {
    let mut map = vec![];
    let mut startpos = Pos { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                startpos = Pos { x, y };
                row.push('F');
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }
    (map, startpos)
}
fn bfs2(map: &[Vec<char>], start: Pos) -> (usize, HashSet<Pos>) {
    let mut que = VecDeque::new();
    let mut visited_map = HashSet::new();
    let mut steps = 0;
    que.push_back(start);
    visited_map.insert(start);
    while !que.is_empty() {
        steps += 1;
        for _ in 0..que.len() {
            let current_node = que.pop_front().unwrap();
            for next_move in valid_next_move(map, current_node) {
                if !visited_map.contains(&next_move) {
                    visited_map.insert(next_move);
                    que.push_back(next_move);
                }
            }
        }
    }
    (steps - 1, visited_map)
}
fn bfs(map: &[Vec<char>], start: Pos) -> (usize, HashSet<Pos>) {
    let mut que = VecDeque::new();
    let mut visited_map = HashSet::new();
    let mut steps = 0;
    que.push_back(start);
    visited_map.insert(start);
    while !que.is_empty() {
        steps += 1;
        for _ in 0..que.len() {
            let current_node = que.pop_front().unwrap();
            for next_move in valid_next_move(map, current_node) {
                if !visited_map.contains(&next_move) {
                    visited_map.insert(next_move);
                    que.push_back(next_move);
                }
            }
        }
    }
    (steps - 1, visited_map)
}
fn valid_next_move(map: &[Vec<char>], pos: Pos) -> Vec<Pos> {
    let mut ret = vec![];
    match map[pos.y][pos.x] {
        '|' => {
            ret.push(pos.north());
            ret.push(pos.south());
        }
        '-' => {
            ret.push(pos.east());
            ret.push(pos.west());
        }
        'L' => {
            ret.push(pos.north());
            ret.push(pos.east());
        }
        'J' => {
            ret.push(pos.north());
            ret.push(pos.west());
        }
        '7' => {
            ret.push(pos.south());
            ret.push(pos.west());
        }
        'F' => {
            ret.push(pos.south());
            ret.push(pos.east());
        }
        _ => (),
    }
    ret
}
fn valid_next_move2(map: &[Vec<char>], pos: Pos) -> Vec<Pos> {
    let mut ret = vec![];
    match map[pos.y][pos.x] {
        '|' => {
            ret.push(pos.north());
            ret.push(pos.south());
        }
        '-' => {
            ret.push(pos.east());
            ret.push(pos.west());
        }
        'L' => {
            ret.push(pos.north());
            ret.push(pos.east());
        }
        'J' => {
            ret.push(pos.north());
            ret.push(pos.west());
        }
        '7' => {
            ret.push(pos.south());
            ret.push(pos.west());
        }
        'F' => {
            ret.push(pos.south());
            ret.push(pos.east());
        }
        _ => (),
    }
    ret
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn north(&self) -> Pos {
        let y = if self.y > 0 { self.y - 1 } else { self.y };
        Pos { y, x: self.x }
    }
    fn south(&self) -> Pos {
        Pos {
            y: self.y + 1,
            x: self.x,
        }
    }
    fn east(&self) -> Pos {
        Pos {
            y: self.y,
            x: self.x + 1,
        }
    }
    fn west(&self) -> Pos {
        let x = if self.x > 0 { self.x - 1 } else { self.x };
        Pos { y: self.y, x }
    }
}
// enum Dir {
//     Up,
//     Down,
//     Left,
//     Right,
// }
// impl Dir {
//     pub fn iterator() -> Iter<'static, Dir> {
//         static DIRECTIONS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
//         DIRECTIONS.iter()
//     }
// }

fn find_inside(map: &[Vec<char>], visited: &HashSet<Pos>) -> usize {
    let mut ret = 0;
    for (y, row) in map.iter().enumerate() {
        let mut wallcount = 0;
        for (x, c) in row.iter().enumerate() {
            if visited.contains(&Pos { x, y }) {
                if c != &'-' {
                    wallcount += 1;
                }
            } else if wallcount % 2 != 0 {
                println!("inside at {y}  {x}, wall: {wallcount}");
                ret += 1;
            }
        }
    }
    ret
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";
    #[test]
    fn it_works() {
        let (map, start) = parse_input(TESTINPUT);
        let (steps, visited) = bfs(&map, start);
        println!("{steps:?}");
        let insides = find_inside(&map, &visited);
        println!("{insides}");
    }
}
