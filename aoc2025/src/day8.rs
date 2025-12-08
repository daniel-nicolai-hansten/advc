use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .map(|ln| ln.split(',').map(|n| n.parse().unwrap()).collect::<Vec<u64>>())
        .map(|v| (v[0], v[1], v[2]))
        .collect()
}
type Point3D = (u64, u64, u64);
#[aoc(day8, part1)]
fn part1(input: &[Point3D]) -> usize {
    let mut dists = Vec::new();
    let mut clusters: Vec<Vec<Point3D>> = Vec::new();
    clusters.extend(input.iter().map(|&p| vec![p]));
    for i in 0..input.len() {
        let point1 = input[i];
        for point2 in &input[i + 1..] {
            let dist = distance_3d(point1, *point2);
            dists.push((point1, *point2, dist));
        }
    }
    dists.sort_by(|a, b| a.2.total_cmp(&b.2));
    #[cfg(test)]
    let mut connected_rem = 10;
    #[cfg(not(test))]
    let mut connected_rem = 1000;
    'outer: loop {
        for (p1, p2, _dist) in &dists {
            if connected_rem == 0 {
                break 'outer;
            }
            let c1 = clusters.iter().position(|c| c.contains(p1));
            let c2 = clusters.iter().position(|c| c.contains(p2));
            match (c1, c2) {
                (Some(i), Some(j)) if i != j => {
                    println!("Merging clusters {} and {}", i, j);
                    let mut c2_points = clusters[j].clone();
                    clusters[i].append(&mut c2_points);
                    clusters.remove(j);
                }
                (Some(i), None) => {
                    println!("Adding point {:?} to cluster {}", p2, i);
                    clusters[i].push(*p2);
                }
                (None, Some(j)) => {
                    println!("Adding point {:?} to cluster {}", p1, j);
                    clusters[j].push(*p1);
                }
                (None, None) => {
                    println!("Creating new cluster with points {:?} and {:?}", p1, p2);
                    clusters.push(vec![*p1, *p2]);
                }
                (Some(i), Some(j)) => println!("Points {:?} and {:?} are already in the same cluster {}", p1, p2, i),
            }
            connected_rem -= 1;
        }
    }
    clusters.sort_by_key(|c| c.len());
    clusters.reverse();
    for (i, c) in clusters[0..3].iter().enumerate() {
        println!("Cluster {}: {} points", i, c.len());
    }
    clusters[0..3].iter().fold(1, |acc, c| acc * c.len())
}
fn distance_3d(a: (u64, u64, u64), b: (u64, u64, u64)) -> f64 {
    let (x, y, z) = a;
    let (x2, y2, z2) = b;

    let dx = (x as f64 - x2 as f64).abs();
    let dy = (y as f64 - y2 as f64).abs();
    let dz = (z as f64 - z2 as f64).abs();

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}
#[aoc(day8, part2)]
fn part2(input: &[(u64, u64, u64)]) -> u64 {
    let mut dists = Vec::new();
    let mut clusters: Vec<Vec<Point3D>> = Vec::new();
    clusters.extend(input.iter().map(|&p| vec![p]));
    for i in 0..input.len() {
        let point1 = input[i];
        for point2 in &input[i + 1..] {
            let dist = distance_3d(point1, *point2);
            dists.push((point1, *point2, dist));
        }
    }
    dists.sort_by(|a, b| a.2.total_cmp(&b.2));
    let (mut last_x1, mut last_x2) = (0, 0);
    'outer: loop {
        for (p1, p2, _dist) in &dists {
            if clusters.len() == 1 {
                break 'outer;
            }
            let c1 = clusters.iter().position(|c| c.contains(p1));
            let c2 = clusters.iter().position(|c| c.contains(p2));
            match (c1, c2) {
                (Some(i), Some(j)) if i != j => {
                    let mut c2_points = clusters[j].clone();
                    clusters[i].append(&mut c2_points);
                    clusters.remove(j);
                }

                _ => continue,
            }
            (last_x1, last_x2) = (p1.0, p2.0);
        }
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
