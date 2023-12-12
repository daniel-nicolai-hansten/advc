use core::cmp::{max, min};
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
    let mut ret = vec![];
    let mut drift = 0;
    let (mut max_x, mut max_y) = (0, 0);
    input.lines().enumerate().map(|(y, line)| line.chars().enumerate().filter(|(_x, c)| c == '#').map(|(x,_c)| ))
    for (y, ln) in input.lines().enumerate() {
        let line = ln.trim_start();
        let mut stars = false;
        max_y = max(y, max_y);
        for (x, c) in line.chars().enumerate() {
            max_x = max(x, max_x);
            match c {
                '#' => {
                    stars = true;
                    ret.push(Pos { y: y + drift, x });
                }
                _ => (),
            }
        }
        if !stars {
            drift += driftval - 1;
        }
    }

    let driftv = driftval -1;
    let mut last_x = 0;
    let mut drift = 0;
    let mut drift_l = |x| {
        if x > last_x + 1 {
            drift += driftv * (x - last_x - 1)
        }
        last_x = x;
        drift
    };
    let mut ret2 = vec![];
    
    for x in 0..=max_x {
        let mut currentstars: Vec<Pos> = ret.iter().filter(|star| star.x == x).map(|star: &Pos| Pos {
            x: star.x + drift_l(star.x),
            y: star.y,
        }).collect();
        ret2.append(&mut currentstars);
    }
    ret2
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
