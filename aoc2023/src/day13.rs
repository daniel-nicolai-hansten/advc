use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Vec<Pos>> {
    let mut ret = vec![];
    let mut tempvec = vec![];
    let (mut y, mut x) = (0, 0);
    for line in input.lines() {
        if line.trim().is_empty() {
            ret.push(tempvec.clone());
            tempvec.clear();
            y = 0;
        } else {
            for c in line.trim_start().chars() {
                match c {
                    '#' => tempvec.push(Pos { x, y }),
                    _ => (),
                }
                x += 1;
            }
            y += 1;
        }

        x = 0;
    }
    ret.push(tempvec.clone());
    ret
}

#[aoc(day13, part1)]
fn part1(mirrors: &[Vec<Pos>]) -> usize {
    let mirrorsfound = find_mirrors(&mirrors, 0);
    let yfound = mirrorsfound
        .iter()
        .filter(|(_, dir)| dir == &Cordinate::Y)
        .map(|(x, _)| x)
        .sum::<usize>();
    let xfound = mirrorsfound
        .iter()
        .filter(|(_, dir)| dir == &Cordinate::X)
        .map(|(x, _)| x)
        .sum::<usize>();
    yfound * 100 + xfound
}

#[aoc(day13, part2)]
fn part2(mirrors: &[Vec<Pos>]) -> usize {
    let mirrorsfound = find_mirrors(&mirrors, 1);
    let yfound = mirrorsfound
        .iter()
        .filter(|(_, dir)| dir == &Cordinate::Y)
        .map(|(x, _)| x)
        .sum::<usize>();
    let xfound = mirrorsfound
        .iter()
        .filter(|(_, dir)| dir == &Cordinate::X)
        .map(|(x, _)| x)
        .sum::<usize>();
    yfound * 100 + xfound
}

fn find_mirrors(mirrors: &[Vec<Pos>], smugeval: usize) -> Vec<(usize, Cordinate)> {
    let mut mirrorsfound = vec![];
    for mirrormap in mirrors.iter() {
        let max_x = mirrormap.iter().max_by_key(|p| p.x).unwrap().x;
        let max_y = mirrormap.iter().max_by_key(|p| p.y).unwrap().y;
        for (max, dir) in [(max_x, Cordinate::X), (max_y, Cordinate::Y)] {
            'outer: for linenum in 0..max {
                let mut i = 0;
                let mut diffval = 0;
                'inner: while i <= linenum {
                    let row1 = linenum - i;
                    let row2 = linenum + i + 1;
                    diffval += cmp_linepair(&mirrormap, row1, row2, &dir);
                    let lastline = row1 == 0 || row2 == max;
                    match (diffval <= smugeval, diffval == smugeval, lastline) {
                        (false, _, _) => break 'inner,
                        (true, true, true) => {
                            mirrorsfound.push((linenum + 1, dir));
                            continue 'outer;
                        }
                        _ => (),
                    }
                    i += 1;
                }
            }
        }
    }
    mirrorsfound
}

fn cmp_linepair(mirrormap: &[Pos], row1: usize, row2: usize, dir: &Cordinate) -> usize {
    let (line1, line2): (Vec<_>, Vec<_>) = match dir {
        Cordinate::X => (
            mirrormap.iter().filter(|p| p.x == row1).map(|p| p.y).sorted().collect(),
            mirrormap.iter().filter(|p| p.x == row2).map(|p| p.y).sorted().collect(),
        ),
        Cordinate::Y => (
            mirrormap.iter().filter(|p| p.y == row1).map(|p| p.x).sorted().collect(),
            mirrormap.iter().filter(|p| p.y == row2).map(|p| p.x).sorted().collect(),
        ),
    };

    if line1 == line2 {
        0
    } else {
        let mut elements = line1.len() + line2.len();
        for itm in &line1 {
            if line2.contains(itm) {
                elements -= 1;
            }
        }
        for itm in &line2 {
            if line1.contains(itm) {
                elements -= 1;
            }
        }
        elements
    }
}
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
enum Cordinate {
    X,
    Y,
}
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

// #[cfg(test)]
// mod tests2 {
//     use super::*;
//     const TESTINPUT: &str = "#.##..##.
//     ..#.##.#.
//     ##......#
//     ##......#
//     ..#.##.#.
//     ..##..##.
//     #.#.##.#.

//     #...##..#
//     #....#..#
//     ..##..###
//     #####.##.
//     #####.##.
//     ..##..###
//     #....#..#";

//     #[test]
//     fn it_works() {
//         let input = parse_input(TESTINPUT);
//         let mirrorsfound = find_mirrors(&input, 1);
//         let yfound = mirrorsfound
//             .iter()
//             .filter(|(_, dir)| dir == &Cordinate::Y)
//             .map(|(x, _)| x)
//             .sum::<usize>();
//         let xfound = mirrorsfound
//             .iter()
//             .filter(|(_, dir)| dir == &Cordinate::X)
//             .map(|(x, _)| x)
//             .sum::<usize>();
//         let sum = yfound * 100 + xfound;
//         assert_eq!(400, sum);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn part1_example() {
//     //     assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
//     // }

//     // #[test]
//     // fn part2_example() {
//     //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
//     // }
// }
