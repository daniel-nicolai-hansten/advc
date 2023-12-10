use rand::Rng;
use std::{collections::HashMap, vec};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    // let input = TESTINPUT;
    let mut symbols = Vec::new();
    let mut numbers = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let mut num = PartNum::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if num.is_set() {
                        let n = num.get_number();
                        for p in &num.pos {
                            numbers.insert(*p, (n, num.id));
                        }
                        num.clear();
                    }
                }
                '0'..='9' => num.add(c, Pos { x, y }),
                _ => {
                    if num.is_set() {
                        let n = num.get_number();
                        for p in &num.pos {
                            numbers.insert(*p, (n, num.id));
                        }
                        num.clear();
                    }
                    symbols.push(Pos { x, y })
                }
            }
        }
        if num.is_set() {
            let n = num.get_number();
            for p in &num.pos {
                numbers.insert(*p, (n, num.id));
            }
            num.clear();
        }
    }
    let mut partnumbers = vec![];
    for sym in symbols {
        for pos in sym.adjacent() {
            if let Some(num) = numbers.get(&pos) {
                partnumbers.push(*num);
            }
        }
    }
    partnumbers.sort_unstable();
    partnumbers.dedup();
    let mut tot: i32 = 0;
    for (n, _) in partnumbers {
        tot += n;
    }
    println!("{tot}")
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn adjacent(&self) -> Vec<Pos> {
        let mut ret = vec![];
        if self.x > 0 {
            if self.y > 0 {
                ret.push(Pos {
                    x: self.x - 1,
                    y: self.y - 1,
                });
            }
            ret.push(Pos {
                x: self.x - 1,
                y: self.y,
            });
            ret.push(Pos {
                x: self.x - 1,
                y: self.y + 1,
            });
        }
        if self.y > 0 {
            ret.push(Pos {
                x: self.x,
                y: self.y - 1,
            });
            ret.push(Pos {
                x: self.x + 1,
                y: self.y - 1,
            });
        }
        ret.push(Pos {
            x: self.x + 1,
            y: self.y,
        });
        ret.push(Pos {
            x: self.x + 1,
            y: self.y + 1,
        });
        ret.push(Pos {
            x: self.x,
            y: self.y + 1,
        });
        ret
    }
}
struct PartNum {
    digits: Vec<char>,
    pos: Vec<Pos>,
    id: usize,
}
impl PartNum {
    fn new() -> Self {
        Self {
            digits: vec![],
            pos: vec![],
            id: rand::thread_rng().gen(),
        }
    }
    fn add(&mut self, c: char, pos: Pos) {
        self.pos.push(pos);
        self.digits.push(c);
    }
    fn is_set(&self) -> bool {
        !self.digits.is_empty()
    }
    fn get_number(&self) -> i32 {
        let txt: String = self.digits.iter().collect();
        i32::from_str_radix(&txt, 10).unwrap()
    }
    fn clear(&mut self) {
        self.digits.clear();
        self.pos.clear();
        self.id = rand::thread_rng().gen();
    }
}
const TESTINPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
