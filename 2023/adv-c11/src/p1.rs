use core::cmp::{max, min};
use core::slice::Iter;
fn main() {
    let input = include_str!("../input.txt");
    let stars = parse_input(input);
    let mut distances = 0;
    let mut pairs = 0;
    for star in &stars {
        let star_result = star.star_distances(&stars);
        pairs += star_result.len();
        distances += star_result.iter().sum::<usize>();
    }
    distances = distances / 2;
    println!("{distances}");
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn north(&self) -> Pos {
        let y = if self.y > 0 { self.y - 1 } else { self.y };
        Pos { y, x: self.x }
    }
    fn south(&self) -> Pos {
        Pos {
            y: self.y + 1,
            x: self.x,
        }
    }
    fn east(&self) -> Pos {
        Pos {
            y: self.y,
            x: self.x + 1,
        }
    }
    fn west(&self) -> Pos {
        let x = if self.x > 0 { self.x - 1 } else { self.x };
        Pos { y: self.y, x }
    }
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
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    pub fn iterator() -> Iter<'static, Dir> {
        static DIRECTIONS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        DIRECTIONS.iter()
    }
}

fn parse_input(input: &str) -> Vec<Pos> {
    let mut ret = vec![];
    let mut drift = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, ln) in input.lines().enumerate() {
        let line = ln.trim_start();
        max_y = max(y, max_y);
        let mut stars = false;
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
            drift += 1;
        }
    }
    let mut ret2 = vec![];
    let mut drift = 0;
    for x in 0..=max_x {
        let mut stars = false;
        for star in &ret {
            if star.x == x {
                stars = true;
                ret2.push(Pos {
                    x: star.x + drift,
                    y: star.y,
                });
            }
        }
        if !stars {
            drift += 1;
        }
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
        let mut pairs = 0;
        for star in &stars {
            println!("{star:?}");
            let star_result = star.star_distances(&stars);
            pairs += star_result.len();
            distances += star_result.iter().sum::<usize>();
        }
        distances = distances / 2;
        println!("{distances} {pairs}");
    }
    #[test]
    fn test_distance() {
        let pos1 = Pos { x: 1, y: 6 };
        let pos2 = Pos { x: 5, y: 11 };
        assert_eq!(pos1.distance(&pos2), 9);
    }
}
