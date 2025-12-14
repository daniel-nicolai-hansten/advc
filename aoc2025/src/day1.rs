use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<(Dir, i64)> {
    input
        .lines()
        .map(|line| {
            let (d, n) = line.split_at(1);
            let dir = match d {
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => panic!("Invalid direction"),
            };
            let num = n.parse::<i64>().unwrap();
            (dir, num)
        })
        .collect()
}
#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

#[aoc(day1, part1)]
fn part1(input: &Vec<(Dir, i64)>) -> usize {
    input
        .iter()
        .scan(50, |acc, (dir, n)| {
            match dir {
                Dir::Left => *acc -= n,
                Dir::Right => *acc += n,
            }
            Some(acc.rem_euclid(100))
        })
        .filter(|&n| n == 0)
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<(Dir, i64)>) -> i64 {
    input
        .iter()
        .scan(50, |acc, (dir, n)| {
            let (pos, dist) = match dir {
                Dir::Left if *acc == 0 => {
                    let pos = *acc - n;
                    (pos, pos.div_euclid(100).abs() - 1)
                }
                Dir::Left => (*acc - n, (*acc - n - 1).div_euclid(100).abs()),
                Dir::Right => (*acc + n, (*acc + n) / 100),
            };
            //            let dist = pos.div_euclid(100).abs();
            println!("Moving {} {:?} from {} to {} crosses {} zeros", n, dir, *acc, pos, dist);
            *acc = pos.rem_euclid(100);
            let extra = match pos == 0 {
                true => 1,
                false => 0,
            };

            println!("res {}", dist + extra);
            Some(dist + extra)
        })
        .sum()
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
L82";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 6);
    }
}
