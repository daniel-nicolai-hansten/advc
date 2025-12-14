use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, digit1, newline, one_of, space1},
    multi::{many0, many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
};
#[aoc_generator(day12)]
fn parse(input: &str) -> (Vec<Piece>, Vec<((usize, usize), Vec<usize>)>) {
    let (_rest, parsed) = parse_map(input).unwrap();
    parsed
}
fn parse_map(input: &str) -> IResult<&str, (Vec<Piece>, Vec<((usize, usize), Vec<usize>)>)> {
    let map_parse = preceded(
        pair(pair(digit1, char(':')), newline),
        separated_list1(newline, many1(one_of(".#").map(|c| c == '#'))),
    );
    let num_parse = |input| digit1.map(|n: &str| n.parse().unwrap()).parse(input);
    let (rest, maps) = separated_list1(many1(newline), map_parse.map(|map| Piece { map })).parse(input)?;
    let (rest, puzzles) = preceded(
        many0(newline),
        separated_list1(
            newline,
            separated_pair(separated_pair(num_parse, char('x'), num_parse), tag(": "), separated_list1(space1, num_parse)),
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
    fn area(&self) -> usize {
        self.map.iter().flat_map(|iter| iter.iter()).filter(|c| **c).count()
    }
}

#[aoc(day12, part1)]
fn part1(input: &(Vec<Piece>, Vec<((usize, usize), Vec<usize>)>)) -> usize {
    let (pieces, puzzles) = input;
    let (mut impossible, mut easy, mut rest) = (0, 0, 0);
    for ((x, y), pieces_req) in puzzles {
        let area = x * y;
        let max_area: usize = pieces_req.iter().map(|n| *n).sum::<usize>() * 9;
        let min_area = pieces_req.iter().enumerate().map(|(i, p)| pieces[i].area() * p).sum();
        match (area >= max_area, area < min_area) {
            (true, _) => easy += 1,
            (_, true) => impossible += 1,
            (false, false) => rest += 1,
        }
    }
    println!("impossible: {}, easy: {}, rest: {}", impossible, easy, rest);
    easy
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
        assert_eq!(part1(&parse(TESTINPUT)), 3);
    }
}
