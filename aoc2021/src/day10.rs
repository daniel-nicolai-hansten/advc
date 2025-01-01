use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[String]) -> u32 {
    let mut ret = vec![];
    'outer: for line in input {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' if stack.pop() == Some('(') => (),
                ']' if stack.pop() == Some('[') => (),
                '}' if stack.pop() == Some('{') => (),
                '>' if stack.pop() == Some('<') => (),
                _ => {
                    ret.push(c);
                    continue 'outer;
                }
            }
        }
    }

    ret.iter().fold(0, |acc, c| {
        acc + match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    })
}

#[aoc(day10, part2)]
fn part2(input: &[String]) -> u64 {
    let mut ret = vec![];
    'outer: for line in input {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' if stack.pop() == Some('(') => (),
                ']' if stack.pop() == Some('[') => (),
                '}' if stack.pop() == Some('{') => (),
                '>' if stack.pop() == Some('<') => (),
                _ => continue 'outer,
            }
        }

        // println!("stack: {:?}", stack);
        ret.push(stack.iter().rev().fold(0, |acc, c| {
            (5 * acc)
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                }
        }))
    }

    ret.sort();
    println!("{:?} {}", ret, ret.len() / 2);
    *ret.get(ret.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 26397);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 288957);
    }
}
