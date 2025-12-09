use aoc_runner_derive::{aoc, aoc_generator};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;
use std::{
    cmp::{max, min},
    collections::VecDeque,
};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|ln| {
            let mut parts = ln.split(',');
            let x = parts.next().unwrap().trim().parse().unwrap();
            let y = parts.next().unwrap().trim().parse().unwrap();
            (x, y)
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[(u64, u64)]) -> u64 {
    let mut max_area = 0;
    for i in 0..input.len() {
        let (p1_x, p1_y) = input[i];
        for (p2_x, p2_y) in &input[i + 1..] {
            // calculate area between p1 and p2
            let dist_x = p1_x.abs_diff(*p2_x) + 1;
            let dist_y = p1_y.abs_diff(*p2_y) + 1;
            let area = dist_x * dist_y;
            // println!(
            //     "Point1: ({},{}) Point2: ({},{}) Dist: ({},{}) => Area: {}",
            //     p1_x, p1_y, p2_x, p2_y, dist_x, dist_y, area
            // );
            max_area = max(area, max_area);
        }
    }
    max_area
}

#[aoc(day9, part2)]
fn part2(input: &[(u64, u64)]) -> u64 {
    let mut max_area = 0;
    let mut map = input.iter().cloned().collect::<HashSet<_>>();

    let total_combinations = input.len() * (input.len() - 1) / 2;
    let pb = ProgressBar::new(total_combinations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("##-"),
    );
    for ((x1, y1), (x2, y2)) in input.iter().tuple_windows() {
        let minx = min(x1, x2);
        let maxx = max(x1, x2);
        let miny = min(y1, y2);
        let maxy = max(y1, y2);

        for x in *minx..*maxx {
            map.insert((x, *miny));
            // println!("Inserting ({},{})", x, miny);
        }
        for y in *miny..*maxy {
            map.insert((*minx, y));
            // println!("Inserting ({},{})", minx, y);
        }
    }
    if let Some(((x1, y1), (x2, y2))) = input.first().zip(input.last()) {
        let minx = min(x1, x2);
        let maxx = max(x1, x2);
        let miny = min(y1, y2);
        let maxy = max(y1, y2);
        for x in *minx..*maxx {
            map.insert((x, *miny));
            // println!("Inserting ({},{})", x, miny);
        }
        for y in *miny..*maxy {
            map.insert((*minx, y));
            // println!("Inserting ({},{})", minx, y);
        }
    }
    println!("Boundary map size: {}", map.len());

    // Find bounding box
    let min_x = input.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = input.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = input.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = input.iter().map(|(_, y)| *y).max().unwrap();

    // Find a point inside using ray casting or pick's theorem
    // let inside_point = find_inside_point(&map, min_x, max_x, min_y, max_y);
    // if let Some((ix, iy)) = inside_point {
    //     println!("Found inside point: ({},{})", ix, iy);
    //     flood_fill(&mut map, (ix, iy), min_x, max_x, min_y, max_y);
    //     println!("After flood fill, map size: {}", map.len());
    // }

    for i in 0..input.len() {
        let (x1, y1) = input[i];
        'outer: for (x2, y2) in &input[i + 1..] {
            pb.inc(1);
            pb.set_message(format!("Processing ({},{}) vs ({},{})", x1, y1, x2, y2));

            let minx = min(x1, *x2);
            let maxx = max(x1, *x2);
            let miny = min(y1, *y2);
            let maxy = max(y1, *y2);
            // check that all points inside the rectangle are in the map
            for x in minx..maxx {
                for y in miny..maxy {
                    if !map.contains(&(x, y)) && !is_inside_polygon_shoelace(x, y, &input) {
                        continue 'outer;
                    }
                    map.insert((x, y));
                }
            }
            let dist_x = x1.abs_diff(*x2) + 1;
            let dist_y = y1.abs_diff(*y2) + 1;
            let area = dist_x * dist_y;
            max_area = max(area, max_area);
        }
    }
    max_area
}

fn find_inside_point(boundary: &HashSet<(u64, u64)>, min_x: u64, max_x: u64, min_y: u64, max_y: u64) -> Option<(u64, u64)> {
    // Try points inside the bounding box
    for y in min_y + 1..max_y {
        for x in min_x + 1..max_x {
            if !boundary.contains(&(x, y)) && is_inside_polygon(x, y, boundary, max_x) {
                return Some((x, y));
            }
        }
    }
    None
}
fn is_inside_polygon(x: u64, y: u64, boundary: &HashSet<(u64, u64)>, max_x: u64) -> bool {
    // Ray casting algorithm - count intersections to the right
    let mut intersections = 0;
    for test_x in x + 1..=max_x {
        if boundary.contains(&(test_x, y)) {
            intersections += 1;
        }
    }
    intersections % 2 == 1
}
fn is_inside_polygon_shoelace(px: u64, py: u64, polygon: &[(u64, u64)]) -> bool {
    let n = polygon.len();
    let mut inside = false;

    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn flood_fill(boundary: &mut HashSet<(u64, u64)>, start: (u64, u64), min_x: u64, max_x: u64, min_y: u64, max_y: u64) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    boundary.insert(start);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in &directions {
            let new_x = (x as i64 + dx) as u64;
            let new_y = (y as i64 + dy) as u64;

            // Check bounds
            if new_x < min_x || new_x > max_x || new_y < min_y || new_y > max_y {
                continue;
            }

            // Skip if it's a boundary or already visited
            if boundary.contains(&(new_x, new_y)) {
                continue;
            }

            boundary.insert((new_x, new_y));
            queue.push_back((new_x, new_y));
        }
    }
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
