use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse<'a>(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|s| {
            let splits: Vec<&str> = s.split_whitespace().collect();
            (
                splits[0].parse::<u32>().unwrap(),
                splits[1].parse::<u32>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1<'a>(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut num = 0;
    let mut list1 = input.0.clone();
    let mut list2 = input.1.clone();
    list1.sort();
    list2.sort();
    for (i, num1) in list1.iter().enumerate() {
        let diff = num1.abs_diff(list2[i]);
        num += diff;
    }
    num
}

#[aoc(day1, part2)]
fn part2<'a>(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut num = 0;
    let list1 = &input.0;
    let list2 = &input.1;
    for num1 in list1 {
        num += num1 * list2.iter().filter(|&x| num1 == x).count() as u32;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn part1_example() {
        let parsed1: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let parsed2: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(parse(TESTINPUT1), (parsed1, parsed2));
        assert_eq!(part1(&parse(TESTINPUT1)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT1)), 31);
    }
}
