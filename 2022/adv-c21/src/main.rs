use nom::bytes::complete::take_until;
use nom::character::complete::digit1;
use nom::sequence::preceded;
use nom::{
    bytes::complete::{tag, take},
    *,
};
use std::collections::HashMap;
use std::fs;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (mut high, mut low) = (false, false);
    let mut lastnum = 0;
    'outer: for i in 3093175982000..3093175983000 {
        let number = i;
        let (monkeylist, mut monkeys) = parse_input(&input);
        let human = Monkey {
            name: usize::from_str_radix("humn", 36).unwrap(),
            num: MonkeyNumber::Num(number),
        };
        monkeys.insert(human.name, human);
        'inner: loop {
            for mnky in &monkeylist {
                let monkey = monkeys.get(&mnky).unwrap();
                let mut res = None;
                match &monkey.num {
                    MonkeyNumber::Num(_) => (),
                    MonkeyNumber::MathOperation((monkey1, monkey2, op)) => {
                        if let MonkeyNumber::Num(mnky1_num) = monkeys.get(&monkey1).unwrap().num {
                            if let MonkeyNumber::Num(mnky2_num) = monkeys.get(&monkey2).unwrap().num
                            {
                                match op {
                                    Operation::Divide => res = Some(mnky1_num / mnky2_num),
                                    Operation::Multiply => res = Some(mnky1_num * mnky2_num),
                                    Operation::Add => res = Some(mnky1_num + mnky2_num),
                                    Operation::Sub => res = Some(mnky1_num - mnky2_num),
                                }
                            }
                        }
                    }
                }
                if let Some(num) = res {
                    if mnky == &usize::from_str_radix("lttc", 36).unwrap() {
                        if num < 89661494901968 {
                            print!(".");
                            high = true;
                        } else if num > 89661494901968 {
                            print!("+");
                            lastnum = number;
                            low = true;
                        } else if num == 89661494901968 {
                            println!("2:{} i: {}", num, i);
                            break 'outer;
                        }
                    }
                    if high && low {
                        println!("number just under {} and over {}", number, lastnum);
                        break 'outer;
                    }
                    let mut newmonkey = monkey.clone();
                    newmonkey.num = MonkeyNumber::Num(num);
                    monkeys.insert(newmonkey.name, newmonkey);
                    if mnky == &usize::from_str_radix("root", 36).unwrap() {
                        break 'inner;
                    }
                }
            }
        }
    }
    //let result = monkeys.get(&usize::from_str_radix("root", 36).unwrap());
    //println!("{:?}", result);
}
#[derive(PartialEq, Debug, Clone)]
struct Monkey {
    name: usize,
    num: MonkeyNumber,
}
#[derive(PartialEq, Debug, Clone)]
enum MonkeyNumber {
    Num(i64),
    MathOperation((usize, usize, Operation)),
}
#[derive(PartialEq, Debug, Clone)]
enum Operation {
    Divide,
    Multiply,
    Add,
    Sub,
}
fn parse_input(input: &str) -> (Vec<usize>, HashMap<usize, Monkey>) {
    let mut monkeylist = HashMap::new();
    let mut monkeylist2 = Vec::new();
    for line in input.lines() {
        if let Ok((_, mnky)) = parse_monkey(line) {
            //println!("{:?}", mnky);
            monkeylist2.push(mnky.name);
            monkeylist.insert(mnky.name, mnky);
        }
    }
    (monkeylist2, monkeylist)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = take_until(": ")(input)?;
    let mut num;
    if input.len() == 13 {
        let (input, monkey1) = preceded(tag(": "), take(4 as usize))(input)?;
        let (input, operationchar) = preceded(tag(" "), take(1 as usize))(input)?;
        let (_input, monkey2) = preceded(tag(" "), take(4 as usize))(input)?;
        num = MonkeyNumber::MathOperation((
            usize::from_str_radix(monkey1, 36).unwrap(),
            usize::from_str_radix(monkey2, 36).unwrap(),
            match operationchar {
                "/" => Operation::Divide,
                "+" => Operation::Add,
                "-" => Operation::Sub,
                "*" => Operation::Multiply,
                _ => Operation::Multiply,
            },
        ));
    } else {
        let (_input, number) = preceded(tag(": "), digit1)(input)?;
        num = MonkeyNumber::Num(i64::from_str_radix(number, 10).unwrap());
    }

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
        println!("{:?}", result2);
        assert_eq!(
            Monkey {
                name: 621705,
                num: MonkeyNumber::Num(5),
            },
            result
        );
    }
}
