use aoc_runner_derive::{aoc, aoc_generator};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;
use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|ln| {
            let (p1, p2) = ln.split_once(',').unwrap();
            let x = p1.trim().parse::<u64>().unwrap();
            let y = p2.trim().parse::<u64>().unwrap();
            (x, y)
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[(u64, u64)]) -> u64 {
    input.iter().tuple_combinations().map(|(p1, p2)| calc_area(p1, p2)).max().unwrap()
}
fn calc_area(p1: &(u64, u64), p2: &(u64, u64)) -> u64 {
    let dist_x = p1.0.abs_diff(p2.0) + 1;
    let dist_y = p1.1.abs_diff(p2.1) + 1;
    dist_x * dist_y
}

#[aoc(day9, part2)]
fn part2(input: &[(u64, u64)]) -> u64 {
    // Group points by coordinates
    let mut by_x: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut by_y: HashMap<u64, Vec<u64>> = HashMap::new();

    for &(x, y) in input {
        by_x.entry(x).or_default().push(y);
        by_y.entry(y).or_default().push(x);
    }

    // Create horizontal and vertical segments
    let mut h_segs = Vec::new();
    let mut v_segs = Vec::new();

    for (x, mut ys) in by_x {
        ys.sort();
        for i in (0..ys.len()).step_by(2) {
            if i + 1 < ys.len() {
                h_segs.push(((x, ys[i]), (x, ys[i + 1])));
            }
        }
    }

    for (y, mut xs) in by_y {
        xs.sort();
        for i in (0..xs.len()).step_by(2) {
            if i + 1 < xs.len() {
                v_segs.push(((xs[i], y), (xs[i + 1], y)));
            }
        }
    }

    let mut max_area = 0;

    // Check all rectangle pairs
    for (p1, p2) in input.iter().tuple_combinations() {
        let (x1, y1) = *p1;
        let (x2, y2) = *p2;
        let minx = x1.min(x2);
        let maxx = x1.max(x2);
        let miny = y1.min(y2);
        let maxy = y1.max(y2);

        let mut works = true;

        // Check horizontal segments
        for &((hx, hy0), (_, hy1)) in &h_segs {
            let hy_min = hy0.min(hy1);
            let hy_max = hy0.max(hy1);

            if hx > minx && hx < maxx {
                let ok = hy_max <= miny || hy_min >= maxy;
                if !ok {
                    works = false;
                    break;
                }
            }
        }

        if !works {
            continue;
        }

        // Check vertical segments
        for &((vx0, vy), (vx1, _)) in &v_segs {
            let vx_min = vx0.min(vx1);
            let vx_max = vx0.max(vx1);

            if vy > miny && vy < maxy {
                let ok = vx_max <= minx || vx_min >= maxx;
                if !ok {
                    works = false;
                    break;
                }
            }
        }

        if works {
            max_area = max_area.max(calc_area(p1, p2));
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 24);
    }
}
