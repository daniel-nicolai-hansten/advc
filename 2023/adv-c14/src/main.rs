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
    let mut rocklist = get_rocklist(&rock);
    move_rocks(&mut rock, Dir::North, &mut rocklist);
    calculate_load(&rocklist, rock.len())
}

fn p2(rockmap: Vec<Vec<Rock>>) -> usize {
    let mut loadarr = vec![];
    let mut rocks2 = rockmap;
    let mut last_i = 0;
    let end = 1000_000_000;
    let mut patternsize = end;
    let mut leftover = 0;
    let mut rocklist = get_rocklist(&rocks2);
    'outer: for i in 1..1000 {
        last_i = i;
        for dir in [Dir::North, Dir::West, Dir::South, Dir::East] {
            move_rocks(&mut rocks2, dir, &mut rocklist);
        }
        let load = calculate_load(&rocklist, rocks2.len());
        loadarr.push(load);

        if loadarr.len() > 64 {
            for windowsize in 3..32 {
                let start1 = loadarr.len() - windowsize;
                let end1 = start1 + windowsize;
                let end2 = start1;
                let start2 = start1 - windowsize;
                let (arr1, arr2) = (&loadarr[start1..end1], &loadarr[start2..end2]);
                if arr1 == arr2 {
                    println!("pattern detected at {start2} of {windowsize}");
                    patternsize = windowsize;
                    leftover = (loadarr.len() - start1) % patternsize;
                    break 'outer;
                }
            }
        }
    }

    let rest = end - last_i + leftover;
    let cycles_left = rest % patternsize;
    let arridx = loadarr.len() - (patternsize - cycles_left + 1);
    println!("{:?}   {}", &loadarr[arridx..], cycles_left);

    loadarr[arridx]
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
fn get_rocklist(rockmap: &[Vec<Rock>]) -> Vec<(usize, usize)> {
    rockmap
        .iter()
        .enumerate()
        .map(|(y, rocks)| {
            rocks
                .iter()
                .enumerate()
                .filter(|(_, rock)| **rock == Rock::Round)
                .map(move |(x, _rock)| (y, x))
        })
        .flatten()
        .collect()
}

fn move_rocks(rockmap: &mut [Vec<Rock>], dir: Dir, rocklist: &mut [(usize, usize)]) {
    let max_x = rockmap[0].len();
    let max_y = rockmap.len();
    let mut rockfunc = |yref: &mut usize, xref: &mut usize| {
        let (y, x) = (*yref, *xref);
        let mut rockmoved = 0;
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
                (*yref, *xref) = (y2, x2);
                rockmoved += 1;
            }
            _ => (),
        }

        rockmoved
    };

    'outer: loop {
        let mut moved = 0;
        for (rocky, rockx) in rocklist.iter_mut() {
            moved += rockfunc(rocky, rockx);
        }
        if moved == 0 {
            break 'outer;
        }
    }
}

fn calculate_load(rocks: &[(usize, usize)], south: usize) -> usize {
    rocks.iter().fold(0, |acc, (y, _x)| acc + south - y)
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
