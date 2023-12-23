use std::char;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<Vec<Terrain>> {
    let mut map = vec![];
    for line in input.lines() {
        let mut ln = vec![];
        for c in line.chars() {
            let trrn = match c {
                '.' => Terrain::Path,
                '#' => Terrain::Forest,
                '^' => Terrain::SteepSlope(Dir::Up),
                '>' => Terrain::SteepSlope(Dir::Right),
                'v' => Terrain::SteepSlope(Dir::Down),
                '<' => Terrain::SteepSlope(Dir::Left),
                _ => panic!("Unknown char"),
            };
            ln.push(trrn);
        }
    }
    map
}

#[aoc(day23, part1)]
fn part1(map: &[Vec<Terrain>]) -> String {
    todo!()
}

#[aoc(day23, part2)]
fn part2(input: &str) -> String {
    todo!()
}
// paths (.), forest (#), and steep slopes (^, >, v, and <).
enum Terrain {
    Path,
    Forest,
    SteepSlope(Dir),
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
