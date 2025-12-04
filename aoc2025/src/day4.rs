use aoc_runner_derive::{aoc, aoc_generator};
use common::pos::{Direction, Pos, Position, PositionError};
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<bool>> {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '@' => true,
                '.' => false,
                c => panic!("Invalid character in input: {}", c),
            });
        }
        map.push(row);
    }
    map
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Vec<bool>>) -> u64 {
    let rows = input.len();
    let cols = input[0].len();
    let mut count = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell {
                let mut neighbors = 0;
                // Check neighbors
                let pos = (y, x);
                for neighbor in pos.all_neighbor_positions() {
                    let (ny, nx) = neighbor;
                    if let Some(true) = input.get(ny).and_then(|r| r.get(nx)) {
                        if input[ny][nx] {
                            neighbors += 1;
                        }
                    }
                }
                if neighbors < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
fn part2(input: &Vec<Vec<bool>>) -> u64 {
    let rows = input.len();
    let cols = input[0].len();
    let mut count = 0;
    let mut pos_removed = vec![];
    loop {
        let mut any_removed = false;
        for (y, row) in input.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell && !pos_removed.contains(&(y, x)) {
                    let mut neighbors = 0;
                    // Check neighbors
                    let pos = (y, x);
                    for neighbor in pos.all_neighbor_positions() {
                        let (ny, nx) = neighbor;
                        if let Some(true) = input.get(ny).and_then(|r| r.get(nx)) {
                            if input[ny][nx] && !pos_removed.contains(&neighbor) {
                                neighbors += 1;
                            }
                        }
                    }
                    if neighbors < 4 {
                        count += 1;
                        pos_removed.push(pos);
                        any_removed = true;
                    }
                }
            }
        }
        if !any_removed {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 13);
    }
}
