use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
#[aoc_generator(day17)]
fn parse(input: &str) -> (HistorianComputer, Vec<u64>) {
    parse2(input).unwrap().1
}
fn parse2(input: &str) -> IResult<&str, (HistorianComputer, Vec<u64>)> {
    let (i, reg_a) = preceded(tag("Register A: "), complete::u64)(input)?;
    let (i, _) = many1(tag("\n"))(i)?;
    let (i, reg_b) = preceded(tag("Register B: "), complete::u64)(i)?;
    let (i, _) = many1(tag("\n"))(i)?;
    let (i, reg_c) = preceded(tag("Register C: "), complete::u64)(i)?;
    let (i, _) = many1(tag("\n"))(i)?;
    let (i, program) = preceded(tag("Program: "), separated_list1(complete::char(','), complete::u64))(i)?;
    Ok((
        i,
        (
            HistorianComputer {
                reg_a,
                reg_b,
                reg_c,
                pc: 0,
                output: Vec::new(),
            },
            program,
        ),
    ))
}

#[aoc(day17, part1)]
fn part1(input: &(HistorianComputer, Vec<u64>)) -> String {
    let (computer, program) = input;
    let mut hpc = computer.clone();
    while let Some(&[opr, arg]) = program.get(hpc.pc()..=hpc.pc() + 1) {
        let opr = Operation::from_optcode(opr);
        hpc.opr(opr, arg);
    }
    let mut out = String::new();
    for c in hpc.output {
        out.push(std::char::from_digit(c as u32, 10).unwrap());
        out.push(',');
    }
    out.pop();
    out
}

#[aoc(day17, part2)]
fn part2(input: &(HistorianComputer, Vec<u64>)) -> u64 {
    let mut res = 0;
    let (computer, program) = input;
    let mut trya = 0;
    'outer: for (prog_num1, prog_num2) in program.iter().rev().tuple_windows() {
        for i in 0..0o100 {
            let mut hpc = computer.clone();
            hpc.reg_a = trya | i;

            while let Some(&[opr, arg]) = program.get(hpc.pc()..=hpc.pc() + 1) {
                let opr = Operation::from_optcode(opr);
                hpc.opr(opr, arg);
            }
            if let Some(num) = hpc.output.get(0..2) {
                match num {
                    &[n1, n2] => {
                        if n1 == *prog_num2 && n2 == *prog_num1 {
                            trya |= i;
                            if &hpc.output == program {
                                res = trya;
                                break 'outer;
                            } else {
                                trya <<= 3;
                                break;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    res
}
#[derive(Debug, Clone)]
struct HistorianComputer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    pc: u64,
    output: Vec<u64>,
}
impl HistorianComputer {
    fn pc(&self) -> usize {
        self.pc as usize
    }
    fn adv(&mut self, arg: u64) {
        let combo = self.combo(arg);
        let denominator = 2_u64.pow(combo as u32);
        self.reg_a = self.reg_a.checked_div(denominator).unwrap_or(0);
        self.pc += 2;
    }
    fn bxl(&mut self, arg: u64) {
        self.reg_b ^= arg;
        self.pc += 2;
    }
    fn bst(&mut self, arg: u64) {
        let combo = self.combo(arg);
        self.reg_b = combo & 0b111;
        self.pc += 2;
    }
    fn jnz(&mut self, arg: u64) {
        if self.reg_a != 0 {
            self.pc = arg;
        } else {
            self.pc += 2;
        }
    }
    fn bxc(&mut self, _arg: u64) {
        self.reg_b ^= self.reg_c;
        self.pc += 2;
    }
    fn out(&mut self, arg: u64) {
        let combo = self.combo(arg) & 0b111;
        self.output.push(combo);
        self.pc += 2;
    }
    fn bdv(&mut self, arg: u64) {
        let combo = self.combo(arg);
        let denominator = 2_u64.pow(combo as u32);
        self.reg_b = self.reg_a.checked_div(denominator).unwrap_or(0);
        self.pc += 2;
    }
    fn cdv(&mut self, arg: u64) {
        let combo = self.combo(arg);
        let denominator = 2_u64.pow(combo as u32);
        self.reg_c = self.reg_a.checked_div(denominator).unwrap_or(0);
        self.pc += 2;
    }
    fn combo(&self, arg: u64) -> u64 {
        match arg {
            0..=3 => arg,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }
    fn opr(&mut self, opr: Operation, arg: u64) {
        match opr {
            Operation::Adv => self.adv(arg),
            Operation::Bxl => self.bxl(arg),
            Operation::Bst => self.bst(arg),
            Operation::Jnz => self.jnz(arg),
            Operation::Bxc => self.bxc(arg),
            Operation::Out => self.out(arg),
            Operation::Bdv => self.bdv(arg),
            Operation::Cdv => self.cdv(arg),
        }
    }
}
enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl Operation {
    fn from_optcode(optcode: u64) -> Self {
        match optcode {
            0 => Operation::Adv,
            1 => Operation::Bxl,
            2 => Operation::Bst,
            3 => Operation::Jnz,
            4 => Operation::Bxc,
            5 => Operation::Out,
            6 => Operation::Bdv,
            7 => Operation::Cdv,
            _ => panic!("Invalid optcode"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    const TESTINPUT1: &str = "Register A: 0
Register B: 0
Register C: 9

Program: 2,6
";
    const TESTINPUT2: &str = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
";
    const TESTINPUT3: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    const TESTINPUT4: &str = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7
";

    const TESTINPUT5: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
    #[test]
    fn part1_example() {
        let (mut computer, program) = parse(TESTINPUT1);
        if let &[opr, arg] = program.get(computer.pc()..=computer.pc() + 1).unwrap() {
            let opr = Operation::from_optcode(opr);
            computer.opr(opr, arg);
        }
        assert_eq!(computer.reg_b, 1);
        assert_eq!(part1(&parse(TESTINPUT)), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part1(&parse(TESTINPUT2)), "0,1,2");
        assert_eq!(part1(&parse(TESTINPUT3)), "4,2,5,6,7,7,7,7,3,1,0");
        let (mut computer, program) = parse(TESTINPUT4);
        if let &[opr, arg] = program.get(computer.pc()..=computer.pc() + 1).unwrap() {
            let opr = Operation::from_optcode(opr);
            computer.opr(opr, arg);
        }
        assert_eq!(computer.reg_b, 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT5)), 117440);
    }
}
