use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<(Direction, u32)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        let dir = &line[0..1];
        let dist: u32 = line[1..].parse().unwrap();
        let direction = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        result.push((direction, dist));
    }
    result
}

#[aoc(day1, part1)]
fn part1(input: &[(Direction, u32)]) -> usize {
    let mut pos: u32 = 50;
    let mut visited = vec![pos];
    for &(dir, dist) in input {
        // println!("At pos {} {:?} {}", pos, dir, dist);
        let dist = dist % 100;
        pos = match dir {
            Direction::Left => pos.checked_sub(dist).unwrap_or({
                let wrap = dist.checked_sub(pos).unwrap_or(0);
                100 - wrap
            }),
            Direction::Right => pos + dist,
        };
        pos = pos % 100;
        visited.push(pos);
    }
    visited.iter().filter(|&&p| p == 0).count()
}

#[aoc(day1, part2)]
fn part2(input: &[(Direction, u32)]) -> usize {
    let mut pos: i32 = 50;
    let mut zeros = 0;
    for &(dir, dist) in input {
        for _ in 0..dist {
            match dir {
                Direction::Left => pos -= 1,
                Direction::Right => pos += 1,
            };
            if pos % 100 == 0 {
                zeros += 1;
            }
            if pos == 0 {
                pos = 100;
            }
            if pos == 101 {
                pos = 1;
            }
        }
        println!("Moved {:?} by {}, new pos {}", dir, dist, pos);
    }
    zeros
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 6);
    }
}
