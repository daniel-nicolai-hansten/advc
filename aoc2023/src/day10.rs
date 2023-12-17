use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};
use std::slice::Iter;

#[aoc_generator(day10)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let (map, start) = parse_input(input);
    let (steps, _visited) = bfs(&map, start);
    steps
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let (map, start) = parse_input(input);
    let (_steps, visited) = bfs(&map, start);
    // draw_map(&map, &visited);
    find_internal(&map, &visited)
}



fn find_internal(map: &[Vec<char>], visited: &HashSet<Pos>) -> usize {
    let mut tot = 0;
    let hr_map = high_res_map(&map, &visited);
    let visited2 = bfs_hr(&hr_map, Pos { x: 0, y: 0 });
    for y in 0..(hr_map.len() / 3) {
        for x in 0..(hr_map[0].len() / 3) {
            let pos = Pos { x, y }.scale();
            if !hr_map[pos.y][pos.x] && !visited2.contains(&pos) {
                tot += 1;
            }
        }
    }
    tot
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Pos) {
    let mut map = vec![];
    let mut startpos = Pos { x: 0, y: 0 };
    for (y, ln) in input.lines().enumerate() {
        let mut row = vec![];
        let line = ln;
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                startpos = Pos { x, y };
                row.push('|');
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }
    (map, startpos)
}
fn high_res_map(map: &[Vec<char>], visited: &HashSet<Pos>) -> Vec<Vec<bool>> {
    let mut ret = vec![];

    let res_y = map.len() * 3;
    let res_x = map[0].len() * 3;
    println!("{res_y}  {res_x}");
    for _ in 0..res_y {
        let mut rowvec = vec![];
        for _ in 0..res_x {
            rowvec.push(false);
        }
        ret.push(rowvec);
    }
    for (y, row) in map.iter().enumerate() {
        let mut higres_visited = vec![];
        for (x, c) in row.iter().enumerate() {
            let pos = Pos { x, y };
            if visited.contains(&pos) {
                match c {
                    '|' => {
                        higres_visited.push(pos.scale().north());
                        higres_visited.push(pos.scale());
                        higres_visited.push(pos.scale().south());
                    }
                    '-' => {
                        higres_visited.push(pos.scale().west());
                        higres_visited.push(pos.scale());
                        higres_visited.push(pos.scale().east());
                    }
                    'L' => {
                        higres_visited.push(pos.scale().north());
                        higres_visited.push(pos.scale());
                        higres_visited.push(pos.scale().east());
                    }
                    'J' => {
                        higres_visited.push(pos.scale().north());
                        higres_visited.push(pos.scale());
                        higres_visited.push(pos.scale().west());
                    }
                    '7' => {
                        higres_visited.push(pos.scale().west());
                        higres_visited.push(pos.scale());
                        higres_visited.push(pos.scale().south());
                    }
                    'F' => {
                        higres_visited.push(pos.scale().east());
                        higres_visited.push(pos.scale());
                        higres_visited.push(pos.scale().south());
                    }
                    _ => (),
                }
            }
        }
        for pos in higres_visited {
            ret[pos.y][pos.x] = true;
        }
    }
    ret
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
fn bfs_hr(map: &[Vec<bool>], start: Pos) -> HashSet<Pos> {
    let mut que = VecDeque::new();
    let mut visited_map = HashSet::new();
    que.push_back(start);
    visited_map.insert(start);
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

fn valid_next_move_hr(map: &[Vec<bool>], from: Pos) -> Vec<Pos> {
    let mut results = vec![];
    let (h, w) = (map.len(), map[0].len());
    for dir in Dir::iterator() {
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
        }
        if next != from && !map[next.y][next.x] {
            results.push(next);
        }
    }
    results
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
    fn scale(&self) -> Pos {
        Pos {
            y: self.y * 3 + 1,
            x: self.x * 3 + 1,
        }
    }
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
#[allow(dead_code)]
fn draw_map(map: &[Vec<char>], visited: &HashSet<Pos>) {
    for (y, line) in map.iter().enumerate() {
        for (x, row) in line.iter().enumerate() {
            let p = match (row, visited.contains(&Pos { x, y })) {
                ('|', true) => '│',
                ('-', true) => '─',
                ('7', true) => '┐',
                ('J', true) => '┘',
                ('F', true) => '┌',
                ('L', true) => '└',
                _ => ' ',
            };
            print!("{p}");
        }
        println!();
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    const TESTINPUT: &str = 
    r".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 20);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 20);
    }
}
