use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

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
    let mut max_area = 0;
    // Create segments by finding pairs of points on same x or y coordinate
    let mut h_segs = Vec::new();
    let mut v_segs = Vec::new();

    // Find horizontal segments (same x coordinate)
    for (p1, p2) in input.iter().circular_tuple_windows() {
        let (x1, y1) = *p1;
        let (x2, y2) = *p2;

        if x1 == x2 {
            // Horizontal segment
            h_segs.push(((x1, y1.min(y2)), (x1, y1.max(y2))));
        }
        if y1 == y2 {
            // Vertical segment
            v_segs.push(((x1.min(x2), y1), (x1.max(x2), y1)));
        }
    }

    // Check all rectangle pairs
    'outer: for (p1, p2) in input.iter().tuple_combinations() {
        let potential_area = calc_area(p1, p2);
        if potential_area <= max_area {
            continue;
        }
        let (x1, y1) = *p1;
        let (x2, y2) = *p2;
        let (minx, maxx) = (x1.min(x2), x1.max(x2));
        let (miny, maxy) = (y1.min(y2), y1.max(y2));

        // Check horizontal segments
        for &((hx, hy0), (_, hy1)) in &h_segs {
            let (hy_min, hy_max) = (hy0.min(hy1), hy0.max(hy1));
            if hx > minx && hx < maxx && !(hy_max <= miny || hy_min >= maxy) {
                continue 'outer;
            }
        }

        // Check vertical segments
        for &((vx0, vy), (vx1, _)) in &v_segs {
            let (vx_min, vx_max) = (vx0.min(vx1), vx0.max(vx1));
            if vy > miny && vy < maxy && !(vx_max <= minx || vx_min >= maxx) {
                continue 'outer;
            }
        }

        max_area = max_area.max(calc_area(p1, p2));
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
