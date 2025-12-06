use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut state = 0;
    let mut ranges = vec![];
    let mut numbers = vec![];
    for line in input.lines() {
        match state {
            0 => {
                if line.trim().is_empty() {
                    state = 1;
                    continue;
                }
                let (n1, n2) = line.split_once('-').unwrap();
                let n1 = n1.parse::<usize>().unwrap();
                let n2 = n2.parse::<usize>().unwrap();
                ranges.push((n1, n2));
            }
            1 => {
                let n = line.parse::<usize>().unwrap();
                numbers.push(n);
            }
            _ => panic!("Invalid state"),
        }
    }
    (ranges, numbers)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let mut count = 0;
    let (ranges, numbers) = input;
    for n in numbers {
        for (start, end) in ranges {
            if n >= start && n <= end {
                count += 1;
                break;
            }
        }
    }
    count
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let mut ranges = input.0.clone();
    let mut merged_ranges: Vec<(usize, usize)> = vec![];
    while ranges.len() != merged_ranges.len() {
        merged_ranges.clear();
        for (start, end) in ranges.iter() {
            let mut merged = false;
            for (mstart, mend) in merged_ranges.iter_mut() {
                if (*start <= *mend + 1 && *end >= *mstart - 1) || (*mstart <= *end + 1 && *mend >= *start - 1) {
                    *mstart = (*mstart).min(*start);
                    *mend = (*mend).max(*end);
                    merged = true;
                    break;
                }
            }
            if !merged {
                merged_ranges.push((*start, *end));
            }
        }
        ranges = merged_ranges.clone();
    }
    merged_ranges.iter().fold(0, |acc, (start, end)| acc + end - start + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 14);
    }
}
