use rustc_hash::FxHashSet as HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
type Pos = (isize, isize);
trait Coord {
    fn x(&self) -> isize;
    fn y(&self) -> isize;
    fn in_bounds(&self, max: Pos) -> bool {
        self.x() >= 0 && self.y() >= 0 && self.x() <= max.0 && self.y() <= max.1
    }
}
impl Coord for Pos {
    fn x(&self) -> isize {
        self.0 as isize
    }
    fn y(&self) -> isize {
        self.1 as isize
    }
}
#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<(char, Pos)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, (x as isize, y as isize))))
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[(char, Pos)]) -> u32 {
    let maxpos = input.iter().max_by_key(|(_, pos)| pos.0).unwrap().1;
    let nodes: Vec<(char, Pos)> = input.iter().filter_map(|(c, d)| c.is_alphanumeric().then(|| (*c, *d))).collect();
    let mut antinodes = HashSet::default();
    for (c, post) in &nodes {
        for (_c2, pos2) in nodes.iter().filter(|(c2, p2)| c2 == c && p2 != post) {
            let diff_pos: Pos = (pos2.0 - post.0, pos2.1 - post.1);
            let antinode = (pos2.0 + diff_pos.0, pos2.1 + diff_pos.1);
            if antinode.in_bounds(maxpos) {
                antinodes.insert(antinode);
            }
        }
    }
    antinodes.len() as u32
}

#[aoc(day8, part2)]
fn part2(input: &[(char, Pos)]) -> u32 {
    let maxpos = input.iter().max_by_key(|(_, pos)| pos.0).unwrap().1;
    let nodes: Vec<(char, Pos)> = input.iter().filter_map(|(c, d)| c.is_alphanumeric().then(|| (*c, *d))).collect();
    let mut antinodes = HashSet::default();
    for (c, post) in &nodes {
        for (_c2, pos2) in nodes.iter().filter(|(c2, p2)| c2 == c && p2 != post) {
            antinodes.insert(*pos2);
            let diff_pos: Pos = (pos2.0 - post.0, pos2.1 - post.1);
            let mut antinode = (pos2.0 + diff_pos.0, pos2.1 + diff_pos.1);
            while antinode.in_bounds(maxpos) {
                antinodes.insert(antinode);
                antinode = (antinode.0 + diff_pos.0, antinode.1 + diff_pos.1);
            }
        }
    }
    antinodes.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT1)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT1)), 34);
    }
}
