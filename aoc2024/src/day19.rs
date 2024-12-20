use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use nom::{
    bytes::complete::tag,
    multi::{many1, separated_list1},
    IResult,
};

#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    prse(input)
        .iter()
        .map(|(_, (a, b))| (a.iter().map(|x| x.to_string()).collect(), b.iter().map(|x| x.to_string()).collect()))
        .next()
        .unwrap()
}
fn prse(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (i, towels) = separated_list1(tag(", "), nom::character::complete::alpha1)(input)?;
    let (i, _) = many1(tag("\n"))(i)?;
    let (i, patterns) = separated_list1(tag("\n"), nom::character::complete::alpha1)(i)?;
    Ok((i, (towels, patterns)))
}

#[aoc(day19, part1)]
fn part1(input: &(Vec<String>, Vec<String>)) -> u64 {
    let mut ret = 0;
    let (towels, patterns) = input;
    for pattern in patterns {
        if chk_ptn(to_static_v(&towels), to_static(pattern)).is_some() {
            ret += 1;
        }
    }
    ret
}

#[aoc(day19, part2)]
fn part2(input: &(Vec<String>, Vec<String>)) -> u64 {
    let mut ret = 0;
    let (towels, patterns) = input;
    for pattern in patterns {
        if let Some(n) = chk_ptn(to_static_v(&towels), to_static(pattern)) {
            ret += n;
        }
    }
    ret
}

fn to_static(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}
fn to_static_v(v: &Vec<String>) -> &'static [&'static str] {
    Box::leak(v.iter().map(|x| to_static(x)).collect::<Vec<&str>>().into_boxed_slice())
}
#[cached]
fn chk_ptn(towels: &'static [&str], pattern: &'static str) -> Option<u64> {
    let mut ret = None;
    for towel in towels {
        ret = match pattern.strip_prefix(towel) {
            Some(pattern_left) if pattern_left.is_empty() => { ret.map_or(Some(1), |n | Some(n + 1 ))},
            Some(pattern_left)  => {chk_ptn(towels, pattern_left).map_or(ret,|v|Some(ret.unwrap_or(0) + v) )}
            _ => ret,
        }
    }
    ret
}



#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 16);
    }
}
