use core::cmp::{max, min};
use itertools::Itertools;
fn main() {
    let input = include_str!("../input.txt");
    let stars_p1 = parse_input(input, 2);
    let mut distances = 0;
    for star in &stars_p1 {
        let star_result = star.star_distances(&stars_p1);
        distances += star_result.iter().sum::<usize>();
    }
    distances = distances / 2;
    println!("p1: {distances}");

    let stars_p2 = parse_input(input, 1000000);
    let mut distances = 0;
    for star in &stars_p2 {
        let star_result = star.star_distances(&stars_p2);
        distances += star_result.iter().sum::<usize>();
    }
    distances = distances / 2;
    println!("p2: {distances}");
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        let x_diff = max(self.x, other.x) - min(self.x, other.x);
        let y_diff = max(self.y, other.y) - min(self.y, other.y);
        x_diff + y_diff
    }
    fn star_distances(&self, stars: &[Pos]) -> Vec<usize> {
        let mut ret = vec![];
        for star in stars {
            if star != self {
                ret.push(self.distance(star));
            }
        }
        ret
    }
}

fn parse_input(input: &str, driftval: usize) -> Vec<Pos> {
    let driftval = driftval - 1;
    let (rets, (xs, ys)): (Vec<Pos>, (Vec<usize>, Vec<usize>)) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (Pos { x, y }, (x, y)))
        })
        .unzip();
    let (mut xdrift, mut xlast) = (0, 0);
    let xs: Vec<(usize, usize)> = xs
        .iter()
        .sorted()
        .dedup()
        .map(|x: &usize| {
            if *x > xlast + 1 {
                xdrift += driftval * (x - xlast - 1)
            }
            xlast = *x;
            (*x, xdrift + x)
        })
        .collect();
    let (mut ydrift, mut ylast) = (0, 0);
    let ys: Vec<(usize, usize)> = ys
        .iter()
        .sorted()
        .dedup()
        .map(|y: &usize| {
            if *y > ylast + 1 {
                ydrift += driftval * (y - ylast - 1)
            }
            ylast = *y;
            (*y, ydrift + y)
        })
        .collect();
    rets.iter()
        .map(|pos: &Pos| {
            let (_, x_drift) = xs.iter().find(|(x, _)| *x == pos.x).unwrap();
            let (_, y_drift) = ys.iter().find(|(y, _)| *y == pos.y).unwrap();
            Pos {
                x: *x_drift,
                y: *y_drift,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;
    const TESTINPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    #[test]
    fn it_works() {
        let stars = parse_input(TESTINPUT, 100);
        let mut distances = 0;
        for star in &stars {
            println!("{star:?}");
            let star_result = star.star_distances(&stars);
            distances += star_result.iter().sum::<usize>();
        }
        distances = distances / 2;
        assert_eq!(distances, 8410)
    }
    #[test]
    fn test_distance() {
        let pos1 = Pos { x: 1, y: 6 };
        let pos2 = Pos { x: 5, y: 11 };
        assert_eq!(pos1.distance(&pos2), 9);
    }
}
