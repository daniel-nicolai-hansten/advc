use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .map(|line| {
            let text: Vec<&str> = line.split(&[',', '-']).collect();
            let start1 = text[0].parse::<i32>().unwrap();
            let end1 = text[1].parse::<i32>().unwrap();
            let start2 = text[2].parse::<i32>().unwrap();
            let end2 = text[3].parse::<i32>().unwrap();
            ((start1, end1), (start2, end2))
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[((i32, i32), (i32, i32))]) -> usize {
    input
        .iter()
        .filter(|((start1, end1), (start2, end2))| {
            (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[((i32, i32), (i32, i32))]) -> usize {
    input
        .iter()
        .filter(|((start1, end1), (start2, end2))| start1 <= end2 && start2 <= end1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST_INPUT)), 4);
    }
}
