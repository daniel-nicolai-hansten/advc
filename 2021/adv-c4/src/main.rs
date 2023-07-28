use std::fs;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    pt1(&input);
    pt2(&input);

}

fn pt1(input: &str) {
    let mut boards = parse_input(&input);
    'outer: for n in MOVES.split(',') {
        for board in &mut boards {
            board.check(u32::from_str_radix(n, 10).unwrap());
            if board.has_bingo() {
                println!("Win at number {}, score: {}", n, board.get_score());
                break 'outer;
            }
        }
    }
}

fn pt2(input: &str) {
    let mut boards = parse_input(&input);
    for _ in 0..1000 {
        'outer: for n in MOVES.split(',') {
            for board in &mut boards {
                board.check(u32::from_str_radix(n, 10).unwrap());
                if board.has_bingo() {
                    break 'outer;
                }
            }
        }
        if boards.len() == 1 && boards[0].has_bingo() {
            break;
        }
        boards = boards.into_iter().filter(|bb| !bb.has_bingo() ).collect();
    }
    println!("Last board score: {}, board: {:?} {:?}", boards[0].get_score(), boards[0].board, boards[0].checked );
}

const MOVES: &str = "92,12,94,64,14,4,99,71,47,59,37,73,29,7,16,32,40,53,30,76,74,39,70,88,55,45,17,0,24,65,35,20,63,68,89,84,33,66,18,50,38,10,83,75,67,42,3,56,82,34,90,46,87,52,49,2,21,62,93,86,25,78,19,57,77,26,81,15,23,31,54,48,98,11,91,85,60,72,8,69,6,22,97,96,80,95,58,36,44,1,51,43,9,61,41,79,5,27,28,13";

fn parse_input(input: &str) -> Vec<BingoBoard> {
    let mut ret: Vec<BingoBoard> = vec![];
    for line in input.split("\n\n") {
        ret.push(line.into());
    }
    ret
}
#[derive(Debug)]
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
mod tests1 {
    use super::*;
    const NUMS: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
    const BOARD1: &str = "3 15  0  2 22
     9 18 13 17  5
     19  8  7 25 23
     20 11 10 24  4
     14 21 16 12  6";
    const BOARD2: &str = "3 15  0  2 22
     9 18 13 17  5
     19  8  7 25 23
     20 11 10 24  4
     14 21 16 12  6";
    const BOARD3: &str = "14 21 17 24  4
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
    fn play_bingo() {
        let mut bingos: Vec<BingoBoard> = vec![BOARD1.into(), BOARD2.into(), BOARD3.into()];
        'outer: for n in NUMS.split(',') {
            for bingo in &mut bingos {
                bingo.check(u32::from_str_radix(n, 10).unwrap());
                if bingo.has_bingo() {
                    println!("Win at number {}, score: {}", n, bingo.get_score());
                    break 'outer;
                }
            }
        }
    }
}
