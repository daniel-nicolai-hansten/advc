use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18, part1)]
fn parse(input: &str) -> Vec<(Dir, u64)> {
    let mut ret = vec![];
    for line in input.lines() {
        let splits: Vec<&str> = line.split_whitespace().collect();
        let dir = match splits[0] {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "D" => Dir::Down,
            "U" => Dir::Up,
            _ => unreachable!(),
        };
        let dist = splits[1].parse::<u64>().unwrap();
        ret.push((dir, dist));
    }
    ret
}
#[aoc_generator(day18, part2)]
fn parse2(input: &str) -> Vec<(Dir, u64)> {
    let mut ret = vec![];
    for line in input.lines() {
        let (_, hex) = line.split_once('#').unwrap();
        let dir = match &hex[5..6] {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            _ => unreachable!(),
        };
        let dist = u64::from_str_radix(&hex[..5], 16).unwrap();
        ret.push((dir, dist));
    }
    ret
}

#[aoc(day18, part1)]
fn part1(input: &[(Dir, u64)]) -> u64 {
    shoelace(input)
}

fn shoelace(input: &[(Dir, u64)]) -> u64 {
    let mut startpos = IPos { x: 0, y: 0 };
    let (mut perim, mut sum) = (0, 0);
    for (dir, num) in input {
        let newpos = startpos.point_move(*dir, *num);
        sum += (startpos.y + newpos.y) * (startpos.x - newpos.x);
        perim += num;
        startpos = newpos;
    }
    perim.wrapping_add_signed(sum) / 2 + 1
}

#[aoc(day18, part2)]
fn part2(input: &[(Dir, u64)]) -> u64 {
    shoelace(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct IPos {
    x: i64,
    y: i64,
}
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
struct Pos {
    x: u64,
    y: u64,
}
impl IPos {
    fn point_move(&self, dir: Dir, dist: u64) -> Self {
        match dir {
            Dir::Right => IPos {
                x: self.x.wrapping_add_unsigned(dist),
                y: self.y,
            },
            Dir::Down => IPos {
                x: self.x,
                y: self.y.wrapping_add_unsigned(dist),
            },
            Dir::Left => IPos {
                x: self.x.wrapping_sub_unsigned(dist),
                y: self.y,
            },
            Dir::Up => IPos {
                x: self.x,
                y: self.y.wrapping_sub_unsigned(dist),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(TESTINPUT)), 952408144115);
    }
}
