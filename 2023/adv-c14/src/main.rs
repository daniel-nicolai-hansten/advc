use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");
    let rocks = parse_input(input);

    let sum1 = p1(rocks.clone());
    let elapsed = now.elapsed();
    println!("P{}:{sum1}, Time: {:.2?}", 1, elapsed);

    let sum2 = p2(rocks);
    let elapsed = now.elapsed();
    println!("P{}:{sum2}, Time: {:.2?}", 2, elapsed);
}

fn p1(rockmap: Vec<Vec<Rock>>) -> usize {
    let mut rock = rockmap;
    move_rocks(&mut rock, Dir::North);
    calculate_load(&rock)
}

fn p2(rockmap: Vec<Vec<Rock>>) -> usize {
    let mut loadarr = vec![];
    let mut rocks2 = rockmap;
    let mut last_i = 0;
    let mut patternsize = 0;
    let mut leftover = 0;
    'outer: for i in 0..1000 {
        last_i = i;
        for dir in [Dir::North, Dir::West, Dir::South, Dir::East] {
            move_rocks(&mut rocks2, dir);
        }
        let load = calculate_load(&rocks2);
        loadarr.push(load);
        if loadarr.len() > 200 {
            for windowsize in 3..50 {
                let start1 = loadarr.len() - (windowsize * 2);
                let end1 = start1 + windowsize;
                let start2 = end1;
                let end2 = start2 + windowsize;
                let (arr1, arr2) = (&loadarr[start1..end1], &loadarr[start2..end2]);
                if arr1 == arr2 {
                    println!("pattern detected at {start1} of {windowsize}");
                    patternsize = windowsize;
                    leftover = (i - start1) % patternsize;
                    break 'outer;
                }
            }
        }
    }
    let end = 1000_000_000;
    let rest = end - last_i + leftover;
    let cycles_left = rest % patternsize;
    let mut load_end = 0;
    for _ in 0..cycles_left {
        for dir in [Dir::North, Dir::West, Dir::South, Dir::East] {
            move_rocks(&mut rocks2, dir);
        }
        let load = calculate_load(&rocks2);
        load_end = load;
    }
    load_end
}

fn parse_input(input: &str) -> Vec<Vec<Rock>> {
    let mut ret = vec![];
    for line in input.lines() {
        let mut ln = vec![];
        for c in line.trim().chars() {
            let rck = match c {
                '#' => Rock::Cube,
                'O' => Rock::Round,
                _ => Rock::None,
            };
            ln.push(rck);
        }
        ret.push(ln);
    }
    ret
}

fn move_rocks(rockmap: &mut [Vec<Rock>], dir: Dir) {
    let max_x = rockmap[0].len();
    let max_y = rockmap.len();
    loop {
        let mut rockmoved = false;
        for y in 0..rockmap.len() {
            for x in 0..rockmap[0].len() {
                if rockmap[y][x] == Rock::Round {
                    let (y2, x2) = match dir {
                        Dir::North => {
                            let mut y2 = y;
                            'inner: for y3 in (0..y).rev() {
                                if rockmap[y3][x] != Rock::None {
                                    break 'inner;
                                }
                                y2 = y3;
                            }
                            (y2, x)
                        }
                        Dir::South => {
                            let mut y2 = y;
                            'inner: for y3 in (y + 1)..max_y {
                                if rockmap[y3][x] != Rock::None {
                                    break 'inner;
                                }
                                y2 = y3;
                            }
                            (y2, x)
                        }
                        Dir::East => {
                            let mut x2 = x;
                            'inner: for x3 in (x + 1)..max_x {
                                if rockmap[y][x3] != Rock::None {
                                    break 'inner;
                                }
                                x2 = x3;
                            }
                            (y, x2)
                        }
                        Dir::West => {
                            let mut x2 = x;
                            'inner: for x3 in (0..x).rev() {
                                if rockmap[y][x3] != Rock::None {
                                    break 'inner;
                                }
                                x2 = x3;
                            }
                            (y, x2)
                        }
                    };
                    match (rockmap[y][x], rockmap[y2][x2]) {
                        (Rock::Round, Rock::None) => {
                            rockmap[y][x] = Rock::None;
                            rockmap[y2][x2] = Rock::Round;
                            rockmoved = true;
                        }
                        _ => (),
                    }
                }
            }
        }
        if !rockmoved {
            break;
        }
    }
}

fn calculate_load(rockmap: &[Vec<Rock>]) -> usize {
    let mut ret = 0;
    let south = rockmap.len();
    for (y, row) in rockmap.iter().enumerate() {
        for c in row {
            match c {
                Rock::Cube => (),
                Rock::Round => ret += south - y,
                Rock::None => (),
            };
        }
    }
    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    West,
    South,
    East,
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2, parse_input, Rock};
    const TESTINPUT: &str = "O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....";
    #[allow(dead_code)]
    fn print_rockmap(rockmap: &[Vec<Rock>]) {
        for row in rockmap {
            for c in row {
                let p = match c {
                    Rock::Cube => "#",
                    Rock::Round => "O",
                    Rock::None => ".",
                };
                print!("{p}");
            }
            println!();
        }
    }
    #[test]
    fn it_works_p1() {
        let rocks = parse_input(TESTINPUT);
        let sum = p1(rocks);
        assert_eq!(136, sum);
    }
    #[test]
    fn it_works_p2() {
        let rocks = parse_input(TESTINPUT);
        let sum1 = p2(rocks);
        assert_eq!(64, sum1);
    }
}
