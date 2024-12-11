use crate::pos::{Coord, Pos};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[aoc(day9, part1)]
fn part1(map: &[Vec<u8>]) -> u32 {
    let mut ret = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let pos = (x, y);
            let val = *c;
            let mut lowest = true;
            for neigh in pos.neighbors(map.len() , line.len())  {
                if c >= &map[neigh.y()][neigh.x()] {
                    lowest = false;
                    break;
                }
            }
            if lowest {
                // println!("{} is lowest", val);
                ret += (val +1) as u32;
            }
        }
    }
    ret
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<u8>]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    #[test]
    fn part1_example() {
        let pos = (2, 5);
        assert_eq!(pos.down(5), None);
        assert_eq!(part1(&parse(TESTINPUT)), 15);
    }

    #[test]
    fn part2_example() {
        // assert_eq!(part2(&parse("<EXAMPLE>")), 1134);
    }
}
