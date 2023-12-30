use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{Matrix6, Matrix6x1, RowVector6};
use num::{Num, NumCast};
#[cfg(test)]
const MIN: usize = 7;
#[cfg(test)]
const MAX: usize = 27;
#[cfg(not(test))]
const MIN: usize = 200000000000000;
#[cfg(not(test))]
const MAX: usize = 400000000000000;

#[derive(Debug, Clone)]
struct Hail<N> {
    pos: Point<N>,
    velvec: Point<N>,
}
// Represents a point in 3D space.
#[derive(Debug, Copy, Clone)]
struct Point<N> {
    x: N,
    y: N,
    z: N,
}
impl<N> Point<N>
where
    N: Num + PartialOrd + NumCast,
{
    fn in_area(&self, min: N, max: N) -> bool {
        self.x >= min && self.y >= min && self.x <= max && self.y <= max
    }

    fn to_f64(&self) -> Point<f64> {
        Point {
            x: self.x.to_f64().unwrap(),
            y: self.y.to_f64().unwrap(),
            z: self.z.to_f64().unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
enum Intersection<N> {
    Point(Point<N>),
    Past(Point<N>),
    All,
    None,
}
impl<N> Hail<N>
where
    N: Num + Copy + std::fmt::Debug + NumCast + PartialOrd,
{
    fn find_crossing(&self, other: &Hail<N>) -> Intersection<f64> {
        let slope_self = self.velvec.y.to_f64().unwrap() / self.velvec.x.to_f64().unwrap();
        let slope_other = other.velvec.y.to_f64().unwrap() / other.velvec.x.to_f64().unwrap();
        let intercept_self = self.pos.y.to_f64().unwrap() - slope_self * self.pos.x.to_f64().unwrap();
        let intercept_other = other.pos.y.to_f64().unwrap() - slope_other * other.pos.x.to_f64().unwrap();

        match (slope_self == slope_other, intercept_self == intercept_other) {
            (true, true) => Intersection::All,
            (true, false) => Intersection::None,
            _ => {
                let x = (intercept_other - intercept_self) / (slope_self - slope_other);
                let y = slope_self * x + intercept_self;
                let point = Point { x, y, z: 0f64 };

                if self.in_past_xy(&point) || other.in_past_xy(&point) {
                    Intersection::Past(point)
                } else {
                    Intersection::Point(point)
                }
            }
        }
    }
    fn in_past_xy(&self, point: &Point<f64>) -> bool {
        let x = (point.x - self.pos.x.to_f64().unwrap()) / self.velvec.x.to_f64().unwrap();
        let y = (point.y - self.pos.y.to_f64().unwrap()) / self.velvec.y.to_f64().unwrap();
        x < 0f64 && y < 0f64
    }
}
#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Hail<i128>> {
    let mut ret = vec![];
    for line in input.lines() {
        let (pos, velvec) = line.split_once('@').unwrap();
        let (x, y, z) = pos.split(',').collect_tuple().unwrap();
        let pos: Point<i128> = Point {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
            z: z.trim().parse().unwrap(),
        };
        let (x, y, z) = velvec.split(',').collect_tuple().unwrap();
        let velvec: Point<i128> = Point {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
            z: z.trim().parse().unwrap(),
        };
        let hail = Hail { pos, velvec };
        ret.push(hail);
    }
    ret
}

#[aoc(day24, part1)]
fn part1(input: &[Hail<i128>]) -> usize {
    let mut tot_crossings = 0;
    for  (hail1, hail2)  in input.iter().tuple_combinations() {
            match hail1.find_crossing(hail2) {
                Intersection::Point(pos) if pos.in_area(MIN as f64, MAX as f64) => {
                    tot_crossings += 1;
                }
                _ => (),
            }
            }
    tot_crossings 
}

#[aoc(day24, part2)]
fn part2(input: &[Hail<i128>]) -> usize {
    let (x, y, z) = p2_nalgrebra(input);
    (x + y + z).round() as usize
}

fn p2_nalgrebra(vectors: &[Hail<i128>]) -> (f64, f64, f64) {
    let p0 = &vectors[0].pos;
    let p1 = &vectors[1].pos;
    let p2 = &vectors[2].pos;
    let v0 = &vectors[0].velvec;
    let v1 = &vectors[1].velvec;
    let v2 = &vectors[2].velvec;

    let b = Matrix6x1::from_row_slice(&[
        ((p0.y * v0.x - p1.y * v1.x) - (p0.x * v0.y - p1.x * v1.y)) as f64,
        ((p0.y * v0.x - p2.y * v2.x) - (p0.x * v0.y - p2.x * v2.y)) as f64,
        ((p0.z * v0.x - p1.z * v1.x) - (p0.x * v0.z - p1.x * v1.z)) as f64,
        ((p0.z * v0.x - p2.z * v2.x) - (p0.x * v0.z - p2.x * v2.z)) as f64,
        ((p0.z * v0.y - p1.z * v1.y) - (p0.y * v0.z - p1.y * v1.z)) as f64,
        ((p0.z * v0.y - p2.z * v2.y) - (p0.y * v0.z - p2.y * v2.z)) as f64,
    ]);
    let (p0, p1, p2, v0, v1, v2) = (
        p0.to_f64(),
        p1.to_f64(),
        p2.to_f64(),
        v0.to_f64(),
        v1.to_f64(),
        v2.to_f64(),
    );

    let a = Matrix6::from_rows(&[
        RowVector6::new(v1.y - v0.y, v0.x - v1.x, 0.0, p0.y - p1.y, p1.x - p0.x, 0.0),
        RowVector6::new(v2.y - v0.y, v0.x - v2.x, 0.0, p0.y - p2.y, p2.x - p0.x, 0.0),
        RowVector6::new(v1.z - v0.z, 0.0, v0.x - v1.x, p0.z - p1.z, 0.0, p1.x - p0.x),
        RowVector6::new(v2.z - v0.z, 0.0, v0.x - v2.x, p0.z - p2.z, 0.0, p2.x - p0.x),
        RowVector6::new(0.0, v1.z - v0.z, v0.y - v1.y, 0.0, p0.z - p1.z, p1.y - p0.y),
        RowVector6::new(0.0, v2.z - v0.z, v0.y - v2.y, 0.0, p0.z - p2.z, p2.y - p0.y),
    ]);

    let r = a.lu().solve(&b).unwrap();
    (r[0], r[1], r[2])
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 2);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 47);
    }
}
