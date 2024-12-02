use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse<'a>(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}
enum state
#[aoc(day2, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    for ln in input {
        for (n1, n2) in ln.iter().windows(2) {
           if n1 >= n2 {
               return n1 - n2;
           }
        }

    }
}

#[aoc(day2, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = r#"
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    "#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("TESTINPUT1")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
