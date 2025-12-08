use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap as HashMap;
use std::cmp::{max, min};
type Point3D = (u32, u32, u32);
#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Point3D> {
    input
        .lines()
        .map(|ln| ln.split(',').map(|n| n.parse().unwrap()).collect::<Vec<u32>>())
        .map(|v| (v[0], v[1], v[2]))
        .collect()
}

#[cfg(test)]
const CONNECTED_TARGET: usize = 10;
#[cfg(not(test))]
const CONNECTED_TARGET: usize = 1000;

#[aoc(day8, part1)]
fn part1(input: &[Point3D]) -> usize {
    let mut clusters: Vec<Vec<Point3D>> = Vec::new();
    clusters.extend(input.iter().map(|&p| vec![p]));
    let mut index: HashMap<Point3D, usize> = input.iter().enumerate().map(|(i, &p)| (p, i)).collect();
    let dists = calc_distances(input);
    for (p1, p2, _dist) in dists.iter().take(CONNECTED_TARGET) {
        match (index.get(p1), index.get(p2)) {
            (Some(&i), Some(&j)) if i != j => {
                let mut source = clusters[j].clone();
                for point in &source {
                    index.get_mut(point).map(|v| *v = i);
                }
                clusters[i].append(&mut source);
                clusters.swap_remove(j);
                index.get_mut(p2).map(|v| *v = i);
                if let Some(moved_data) = clusters.get(j) {
                    for point in moved_data {
                        index.get_mut(point).map(|v| *v = j);
                    }
                }
            }
            _ => (),
        }
    }
    clusters.sort_by_key(|c| c.len());
    clusters.iter().rev().take(3).fold(1, |acc, c| acc * c.len())
}

fn distance_3d(a: Point3D, b: Point3D) -> i64 {
    let (x, y, z) = a;
    let (x2, y2, z2) = b;
    let dx = (x as i64 - x2 as i64).abs();
    let dy = (y as i64 - y2 as i64).abs();
    let dz = (z as i64 - z2 as i64).abs();
    dx.pow(2) + dy.pow(2) + dz.pow(2)
}

fn calc_distances(input: &[Point3D]) -> Vec<(Point3D, Point3D, i64)> {
    let mut dists = Vec::with_capacity(input.len() * (input.len() - 1) / 2);
    for i in 0..input.len() {
        let point1 = input[i];
        for point2 in &input[i + 1..] {
            let dist = distance_3d(point1, *point2);
            dists.push((point1, *point2, dist));
        }
    }
    dists.sort_unstable_by_key(|(_p1, _p2, dist)| *dist);
    dists
}

#[aoc(day8, part2)]
fn part2(input: &[Point3D]) -> u32 {
    let mut clusters: Vec<Vec<Point3D>> = Vec::new();
    let mut index: HashMap<Point3D, usize> = input.iter().enumerate().map(|(i, &p)| (p, i)).collect();
    clusters.extend(input.iter().map(|&p| vec![p]));
    let dists = calc_distances(input);
    let (mut last_x1, mut last_x2) = (0, 0);
    for (p1, p2, _dist) in &dists {
        match (index.get(p1), index.get(p2)) {
            (Some(&i), Some(&j)) if i != j => {
                let (source_idx, target_idx) = (max(i, j), min(i, j));
                let mut source = clusters.swap_remove(source_idx);
                source.iter().for_each(|p| index.get_mut(p).map(|v| *v = target_idx).unwrap()); //Reindex points to move
                clusters[target_idx].append(&mut source);
                clusters.get(source_idx).and_then(|mov| {
                    // reindex the swapped cluster
                    mov.iter().for_each(|p| index.get_mut(p).map(|v| *v = source_idx).unwrap());
                    Some(())
                });
            }
            _ if clusters.len() == 1 => break,
            _ => continue,
        }
        (last_x1, last_x2) = (p1.0, p2.0);
    }
    last_x1 * last_x2
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 40);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 25272);
    }
}
