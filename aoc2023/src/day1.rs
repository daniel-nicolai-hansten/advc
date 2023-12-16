use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse<'a>(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day1, part1)]
fn part1<'a>(input: &[String]) -> i32 {
    let mut num = 0;
    for line in input {
        let mut first = 0;
        let mut last = 0;
        'charloop: for c in line.chars() {
            match c {
                '0'..='9' => {
                    first = i32::from_str_radix(&c.to_string(), 10).unwrap();
                    break 'charloop;
                }
                _ => (),
            }
        }
        'charloop: for c in line.chars().rev() {
            match c {
                '0'..='9' => {
                    last = i32::from_str_radix(&c.to_string(), 10).unwrap();
                    break 'charloop;
                }
                _ => (),
            }
        }
        num += i32::from_str_radix(&format!("{first}{last}"), 10).unwrap();
    }
    num
}

#[aoc(day1, part2)]
fn part2<'a>(input: &[String]) -> i32 {
    let mut num = 0;
    for l in input {
        let line = format!("{l}        ");
        let mut first = 0;
        let mut last = 0;
        'charloop: for i in 0..line.len() {
            if let Some(c) = line[i..=i].chars().next() {
                match c {
                    '0'..='9' => {
                        first = i32::from_str_radix(&c.to_string(), 10).unwrap();
                        break 'charloop;
                    }
                    _ => (),
                }
            }
            if i + 3 < line.len() && line[i..i + 3].contains("one") {
                first = 1;
                break 'charloop;
            }
            if i + 3 < line.len() && line[i..i + 3].contains("two") {
                first = 2;
                break 'charloop;
            }
            if i + 5 < line.len() && line[i..i + 5].contains("three") {
                first = 3;
                break 'charloop;
            }
            if i + 4 < line.len() && line[i..i + 4].contains("four") {
                first = 4;
                break 'charloop;
            }
            if i + 4 < line.len() && line[i..i + 4].contains("five") {
                first = 5;
                break 'charloop;
            }
            if i + 3 < line.len() && line[i..i + 3].contains("six") {
                first = 6;
                break 'charloop;
            }
            if i + 5 < line.len() && line[i..i + 5].contains("seven") {
                first = 7;
                break 'charloop;
            }
            if i + 5 < line.len() && line[i..i + 5].contains("eight") {
                first = 8;
                break 'charloop;
            }
            if i + 4 < line.len() && line[i..i + 4].contains("nine") {
                first = 9;
                break 'charloop;
            }
        }

        'charloop: for i in (0..line.len()).rev() {
            for c in line[i - 4..=i].chars() {
                match c {
                    '0'..='9' => {
                        last = i32::from_str_radix(&c.to_string(), 10).unwrap();
                        break 'charloop;
                    }
                    _ => (),
                }
            }
            if i > 3 {
                if line[i - 4..=i].contains("one") {
                    last = 1;
                    break 'charloop;
                }
                if line[i - 4..=i].contains("two") {
                    last = 2;
                    break 'charloop;
                }
                if line[i - 4..=i].contains("three") {
                    last = 3;
                    break 'charloop;
                }
                if line[i - 4..=i].contains("four") {
                    last = 4;
                    break 'charloop;
                }
                if line[i - 4..=i].contains("five") {
                    last = 5;
                    break 'charloop;
                }
                if line[i - 4..=i].contains("six") {
                    last = 6;
                    break 'charloop;
                }
                if line[i - 4..=i].contains("seven") {
                    last = 7;
                    break 'charloop;
                }
                if line[i - 4..=i].contains("eight") {
                    last = 8;
                    break 'charloop;
                }
                if line[i - 4..=i].contains("nine") {
                    last = 9;
                    break 'charloop;
                }
                // println!("{}", &line[i - 4..=i]);
            }
        }
        // println!("found: {first}{last} on line {line}");
        num += i32::from_str_radix(&format!("{first}{last}"), 10).unwrap();
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 20);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 20);
    }
}
