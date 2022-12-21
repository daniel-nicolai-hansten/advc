use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete,
    error::context,
    sequence::{delimited, separated_pair},
    *,
};
fn main() {
    println!("Hello, world!");
}
#[derive(PartialEq, Debug)]
struct Monkey {
    name: usize,
    num: MonkeyNumber,
}
#[derive(PartialEq, Debug)]
enum MonkeyNumber {
    Num(i64),
    MathOperation((usize, usize, Operation)),
}
#[derive(PartialEq, Debug)]
enum Operation {
    Divide,
    Multiply,
    Add,
    Sub,
}
fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for line in input.lines() {}
    monkeys
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = take_until(": ")(input)?;
    let (_, num): (&str,  MonkeyNumber) = alt( (preceded(tag(": "), digit1), preceded(tag(": "), alpha1) )).map(|num: &str| {
        if num.len() == 11 {
            MonkeyNumber::Num(i64::from_str_radix("2", 10).unwrap())
        } else {
            MonkeyNumber::Num(i64::from_str_radix(num, 10).unwrap())
        }
    })(input)?;
    // let (input, clay) =
    //     delimited(tag(" Each clay robot costs "), complete::u64, tag(" ore."))(input)?;
    // let (input, obsidian) = delimited(
    //     tag(" Each obsidian robot costs "),
    //     separated_pair(complete::u64, tag(" ore and "), complete::u64).map(|pair| {
    //         ObsidianRequirements {
    //             ore: pair.0 as usize,
    //             clay: pair.1 as usize,
    //         }
    //     }),
    //     tag(" clay."),
    // )(input)?;
    // let (input, geode) = delimited(
    //     tag(" Each geode robot costs "),
    //     separated_pair(complete::u64, tag(" ore and "), complete::u64).map(|pair| {
    //         GeodeRequirements {
    //             ore: pair.0 as usize,
    //             obsidian: pair.1 as usize,
    //         }
    //     }),
    //     tag(" obsidian."),
    // )(input)?;
    Ok((
        input,
        Monkey {
            name: usize::from_str_radix(id, 36).unwrap(),
            num: num,
        },
    ))
}

const TESTINPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let (_, result) = parse_monkey("dbpl: 5").unwrap();
        let (_, result2) = parse_monkey("pppw: cczh / lfqf").unwrap();
        assert_eq!(
            Monkey {
                name: 621705,
                num: MonkeyNumber::Num(5),
            },
            result
        );
    }
}
