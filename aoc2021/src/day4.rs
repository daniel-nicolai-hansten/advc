use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, error::Error, multi::separated_list0};
#[aoc_generator(day4)]
fn parse(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let (o, moves) = separated_list0(tag(","), nom::character::complete::u32::<&str, Error<&str>>)(input).unwrap();
    let mut bingoboards = vec![];
    for line in o.split("\n\n") {
        if line.is_empty() {
            continue;
        }
        bingoboards.push(line.into());
    }
    (moves, bingoboards)
}

#[aoc(day4, part1)]
fn part1(input: &(Vec<u32>, Vec<BingoBoard>)) -> u32 {
    let mut boards = input.1.clone();
    let moves = &input.0;
    let mut ret = 0;
    'outer: for n in moves {
        for board in &mut boards {
            board.check(*n);
            if board.has_bingo() {
                ret = board.get_score();
                break 'outer;
            }
        }
    }
    ret
}

#[aoc(day4, part2)]
fn part2(input: &(Vec<u32>, Vec<BingoBoard>)) -> u32 {
    let mut boards = input.1.clone();
    let moves = &input.0;
    for _ in 0..1000 {
        'outer: for n in moves {
            for board in &mut boards {
                board.check(*n);
                if board.has_bingo() {
                    break 'outer;
                }
            }
        }
        if boards.len() == 1 && boards[0].has_bingo() {
            break;
        }
        boards = boards.into_iter().filter(|bb| !bb.has_bingo()).collect();
    }
    boards[0].get_score()
}

#[derive(Debug, Clone)]
struct BingoBoard {
    lastnum: u32,
    board: [[u32; 5]; 5],
    checked: [[bool; 5]; 5],
    bingo: bool,
}

impl BingoBoard {
    fn new() -> Self {
        BingoBoard {
            lastnum: 0,
            board: [[0; 5]; 5],
            checked: [[false; 5]; 5],
            bingo: false,
        }
    }
    #[cfg(test)]
    fn get(&self, y: usize, x: usize) -> u32 {
        self.board[y][x]
    }
    fn check(&mut self, n: u32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.board[y][x] == n {
                    self.checked[y][x] = true;
                    self.lastnum = n;
                    self.bingo = self.calculate_bingo()
                }
            }
        }
    }
    fn calculate_bingo(&self) -> bool {
        let mut bingo = false;
        for x in 0..5 {
            bingo = true;
            for y in 0..5 {
                if !self.checked[y][x] {
                    bingo = false;
                    break;
                }
            }
            if bingo {
                return bingo;
            }
        }
        for y in 0..5 {
            bingo = true;
            for x in 0..5 {
                if !self.checked[y][x] {
                    bingo = false;
                    break;
                }
            }
            if bingo {
                return bingo;
            }
        }
        bingo
    }
    fn has_bingo(&self) -> bool {
        self.bingo
    }
    fn get_score(&self) -> u32 {
        let mut sum = 0;
        if self.has_bingo() {
            for y in 0..5 {
                for x in 0..5 {
                    if !self.checked[y][x] {
                        sum += self.board[y][x];
                    }
                }
            }
        }
        sum * self.lastnum
    }
}

impl From<&str> for BingoBoard {
    fn from(s: &str) -> Self {
        let mut ret = BingoBoard::new();
        for (y, line) in s.lines().enumerate() {
            for (x, n) in line.split_ascii_whitespace().enumerate() {
                ret.board[y][x] = u32::from_str_radix(n, 10).unwrap();
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOARD1: &str = "3 15  0  2 22
     9 18 13 17  5
     19  8  7 25 23
     20 11 10 24  4
     14 21 16 12  6";

    const BOARD3: &str = "14 21 17 24  4
     10 16 15  9 19
     18  8 23 26 20
     22 11 13  6  5
      2  0 12  3  7";
    const TESTINPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    #[test]
    fn parse_bingoboard() {
        let bb: BingoBoard = BOARD1.into();
        assert_eq!(bb.get(0, 0), 3);
        assert_eq!(bb.get(2, 0), 19);
    }
    #[test]
    fn has_bingo() {
        let mut bb1: BingoBoard = BOARD1.into();
        let mut bb2: BingoBoard = BOARD3.into();

        assert!(!bb1.has_bingo());
        for i in [3, 15, 0, 2, 22] {
            bb1.check(i);
        }
        for i in [14, 10, 18, 22, 2] {
            bb2.check(i);
        }

        assert!(bb2.has_bingo());
        assert!(bb1.has_bingo());
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 4512);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 1924);
    }
}
