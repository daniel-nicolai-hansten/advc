use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use common::pos::Pos;
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<bool>> {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c == '@');
        }
        map.push(row);
    }
    map
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Vec<bool>>) -> u64 {
    let mut count = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell {
                let mut neighbors = 0;
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
    let mut count = 0;
    let mut input = input.clone();
    // seed work queue with all '@' positions
    let mut wq: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().filter_map(move |(x, &cell)| cell.then_some((x, y))))
        .collect();
    while let Some(pos) = wq.pop() {
        if !input[pos.1][pos.0] {
            continue;
        }
        // filter for neighbors within bounds that are still '@'
        let neighbors = pos
            .all_neighbor_positions()
            .iter()
            .filter_map(|&(nx, ny)| input.get(ny).and_then(|r| r.get(nx).and_then(|p| p.then_some((nx, ny)))))
            .collect::<Vec<_>>();
        if neighbors.len() < 4 {
            count += 1;
            input[pos.1][pos.0] = false;
            wq.extend(neighbors);
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
        assert_eq!(part2(&parse(TESTINPUT)), 43);
    }
}
