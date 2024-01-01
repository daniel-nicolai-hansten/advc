fn main() {
    let mut moves = vec![None; 64];
    for (i, name) in HINT {
        moves[i - 1] = Some(Hest::tp(name));
    }   
    let result = solve(moves);
    for mov in result.unwrap() {
        println!("{}", mov.unwrap().lp());
    }
}
use itertools::Itertools;
fn solve(brett: Vec<Option<Hest>>) -> Option<Vec<Option<Hest>>> {
    // println!("{:?}", brett.iter().filter( |x| x.is_some()).count());
          for ((_i1, m1), (i2, m2), (_i3, m3)) in brett.iter().enumerate().tuple_windows() {
        match (m1, m2, m3) {
            (Some(_), Some(_), Some(_)) => (),
            (Some(move1), None, Some(move3)) => {
                for nxtmove in move1.flytts() {
                    if nxtmove.flytts().contains(move3) && !brett.contains(&Some(nxtmove)) {
                        let mut brett_cpy = brett.clone();
                        brett_cpy[i2] = Some(nxtmove);
                        let ret = solve(brett_cpy);
                        if ret.is_some() {
                            return ret;
                        } else {
                            continue;
                        }
                    }
                }
            }
            (Some(move1), None, None) => {
                for nxtmove in move1.flytts() {
                    if !brett.contains(&Some(nxtmove)) {
                        let mut brett_cpy = brett.clone();
                        brett_cpy[i2] = Some(nxtmove);
                        let ret = solve(brett_cpy);
                        if ret.is_some() {
                            return ret;
                        } else {
                            continue;
                        }
                    }
                }
            }
            _ => (),
        }

    }
    if brett.iter().all(|x| x.is_some()) {
        return Some(brett);
    } else {
        return None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hest {
    rad: u8,
    col: u8,
}
impl Hest {
    fn lp(&self) -> String {
        let base = 'a';
        let rad = (base as u8 + self.rad) as char;
        let col = (self.col + 1).to_string();
        rad.to_string() + &col
    }
    fn tp(pos: &str) -> Hest {
        let (rads, cols) = pos.split_at(1);
        let col = cols.parse::<u8>().unwrap() - 1;
        let rad = rads.chars().next().unwrap() as u8 - 'a' as u8;
        Hest { rad, col }
    }
    fn flytts(&self) -> Vec<Hest> {
        let mut ret = vec![];
        for i in 0..8 {
            match (self.rad, self.col, i) {
                (2..=7, 1..=7, 0) => ret.push(Hest {
                    rad: self.rad - 2,
                    col: self.col - 1,
                }),
                (2..=7, 0..=6, 1) => ret.push(Hest {
                    rad: self.rad - 2,
                    col: self.col + 1,
                }),
                (0..=5, 1..=7, 2) => ret.push(Hest {
                    rad: self.rad + 2,
                    col: self.col - 1,
                }),
                (0..=5, 0..=6, 3) => ret.push(Hest {
                    rad: self.rad + 2,
                    col: self.col + 1,
                }),
                (1..=7, 2..=7, 4) => ret.push(Hest {
                    rad: self.rad - 1,
                    col: self.col - 2,
                }),
                (1..=7, 0..=5, 5) => ret.push(Hest {
                    rad: self.rad - 1,
                    col: self.col + 2,
                }),
                (0..=6, 2..=7, 6) => ret.push(Hest {
                    rad: self.rad + 1,
                    col: self.col - 2,
                }),
                (0..=6, 0..=5, 7) => ret.push(Hest {
                    rad: self.rad + 1,
                    col: self.col + 2,
                }),
                _ => (),
            };
        }
        ret
    }
}
const HINT: [(usize, &str); 18] = [
    (53, "b8"),
    (38, "g8"),
    (33, "c7"),
    (58, "f7"),
    (4, "h7"),
    (19, "d6"),
    (34, "b5"),
    (17, "b2"),
    (50, "a2"),
    (14, "f5"),
    (26, "h5"),
    (22, "a4"),
    (64, "f3"),
    (11, "g3"),
    (2, "h3"),
    (41, "h2"),
    (1, "g1"),
    (44, "b1"),
];
