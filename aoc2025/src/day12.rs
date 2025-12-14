use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, digit1, newline, one_of, space1, u64 as nom_u64},
    multi::{many0, many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
};
#[aoc_generator(day12)]
fn parse(input: &str) -> (Vec<Piece>, Vec<((u64, u64), Vec<u64>)>) {
    let (_rest, parsed) = parse_map(input).unwrap();
    parsed
}
fn parse_map(input: &str) -> IResult<&str, (Vec<Piece>, Vec<((u64, u64), Vec<u64>)>)> {
    let map_parse = preceded(
        pair(pair(digit1, char(':')), newline),
        separated_list1(newline, many1(one_of(".#").map(|c| c == '#'))),
    );
    let (rest, maps) = separated_list1(many1(newline), map_parse.map(|map| Piece { map })).parse(input)?;
    let (rest, puzzles) = preceded(
        many0(newline),
        separated_list1(
            newline,
            separated_pair(separated_pair(nom_u64, char('x'), nom_u64), tag(": "), separated_list1(space1, nom_u64)),
        ),
    )
    .parse(rest)?;
    //println!("maps: {:?}", maps);
    //println!("rest: {}", rest);
    Ok((rest, (maps, puzzles)))
}
#[derive(Debug)]
struct Piece {
    map: Vec<Vec<bool>>,
}
impl Piece {
    fn area(&self) -> u64 {
        self.map.iter().flat_map(|iter| iter.iter()).filter(|c| **c).count() as u64
    }
    fn rotate(&self, times: usize) -> Piece {
        let mut new_map = self.map.clone();
        for _ in 0..times {
            let rows = new_map.len();
            let cols = new_map[0].len();
            let mut rotated = vec![vec![false; rows]; cols];
            for r in 0..rows {
                for c in 0..cols {
                    rotated[c][rows - 1 - r] = new_map[r][c];
                }
            }
            new_map = rotated;
        }
        Piece { map: new_map }
    }
}

#[aoc(day12, part1)]
fn part1(input: &(Vec<Piece>, Vec<((u64, u64), Vec<u64>)>)) -> u64 {
    let (pieces, puzzles) = input;
    let (mut impossible, mut easy, mut rest) = (0, 0, 0);
    let mut tobe_tested = Vec::new();
    for ((x, y), pieces_req) in puzzles {
        let area = x * y;
        let max_area: u64 = pieces_req.iter().map(|n| *n).sum::<u64>() * 9;
        let min_area = pieces_req.iter().enumerate().map(|(i, p)| pieces[i].area() * p).sum();
        match (area >= max_area, area < min_area) {
            (true, _) => easy += 1,
            (_, true) => impossible += 1,
            (false, false) => {
                rest += 1;
                tobe_tested.push(((x, y), pieces_req));
            }
        }
    }
    let mut solvable = easy;
    println!("impossible: {}, easy: {}, rest: {}", impossible, easy, rest);
    for ((x, y), pieces_req) in tobe_tested {
        let board = vec![vec![false; *x as usize]; *y as usize];
        if solve_puzzle(board, pieces_req.clone(), pieces) {
            solvable += 1;
        }
    }
    solvable
}
fn solve_puzzle(board: Vec<Vec<bool>>, pieces_remaining: Vec<u64>, pieces: &[Piece]) -> bool {
    if pieces_remaining.iter().all(|n| *n == 0) {
        return true;
    }
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            for (i, _count) in pieces_remaining.iter().enumerate().filter(|(_, n)| **n != 0) {
                'outer: for rotation in 0..4 {
                    let rotated_piece = pieces[i].rotate(rotation);
                    let mut new_board = board.clone();
                    for (row_idx, piece_row) in rotated_piece.map.iter().enumerate() {
                        for (col_idx, _cell) in piece_row.iter().enumerate().filter(|(_, c)| **c) {
                            let ny = y + row_idx;
                            let nx = x + col_idx;
                            if !(ny >= board.len() || nx >= board[0].len() || board[ny][nx]) {
                                new_board[ny][nx] = true;
                            } else {
                                continue 'outer; // Piece doesn't fit here
                            }
                        }
                    }
                    let mut new_pieces = pieces_remaining.clone();
                    new_pieces[i] -= 1;
                    return solve_puzzle(new_board, new_pieces, pieces);
                }
                //return false; // No piece could be placed here
            }
        }
    }
    false
}
fn print_board(board: &Vec<Vec<bool>>) {
    for row in board {
        for &cell in row {
            if cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 2);
    }
}
