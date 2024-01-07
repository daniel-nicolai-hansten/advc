use aoc_runner_derive::{aoc, aoc_generator};
use rand::Rng;
use rustc_hash::FxHashMap as HashMap;
use std::vec;

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let mut symbols = Vec::new();
    let mut numbers = HashMap::default();
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
    tot
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let mut gears = Vec::new();
    let mut numbers = HashMap::default();
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
                '*' => {
                    if num.is_set() {
                        let n = num.get_number();
                        for p in &num.pos {
                            numbers.insert(*p, (n, num.id));
                        }
                        num.clear();
                    }
                    gears.push(Pos { x, y })
                }
                _ => {
                    if num.is_set() {
                        let n = num.get_number();
                        for p in &num.pos {
                            numbers.insert(*p, (n, num.id));
                        }
                        num.clear();
                    }
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
    for gear in gears {
        let mut tmplist = vec![];
        for pos in gear.adjacent() {
            if let Some(num) = numbers.get(&pos) {
                tmplist.push(*num);
            }
        }
        tmplist.sort_unstable();
        tmplist.dedup();
        if tmplist.len() == 2 {
            let mut gear_ratio = 1;
            for (itm, _) in tmplist {
                gear_ratio *= itm;
            }
            partnumbers.push(gear_ratio);
        }
    }
    let mut tot: i32 = 0;
    for n in partnumbers {
        tot += n;
    }
    tot
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

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 467835);
    }
}
