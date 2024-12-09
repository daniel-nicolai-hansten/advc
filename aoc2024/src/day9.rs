use aoc_runner_derive::{aoc, aoc_generator};
use nom::character::complete::tab;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dsk {
    File(u32),
    Free,
}
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Dsk> {
    let mut ret = vec![];
    for (n, c) in input.chars().enumerate() {
        for _ in 0..c.to_digit(10).unwrap() {
            let fs = match n % 2 {
                0 => Dsk::File(n as u32 / 2),
                1 => Dsk::Free,
                _ => panic!("Invalid input"),
            };
            ret.push(fs);
        }
    }
    ret
}

#[aoc(day9, part1)]
fn part1(input: &[Dsk]) -> usize {
    let mut ret = 0;
    let mut dsk = input.to_vec();
    let (mut head, mut tail) = (0, dsk.len() - 1);
    'outer: loop {
        while dsk[head] != Dsk::Free {
            head += 1;
            if head >= tail {
                break 'outer;
            }
        }
        while dsk[tail] == Dsk::Free {
            tail -= 1;
            if head >= tail {
                break 'outer;
            }
        }
        dsk.swap(head, tail);
    }
    dsk.iter().enumerate().fold(0, |acc, (idx, f)| match f {
        Dsk::File(n) => (idx * *n as usize) + acc,
        Dsk::Free => acc,
    })
}

#[aoc(day9, part2)]
fn part2(input: &[Dsk]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "2333133121414131402";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        // assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
