use std::fs;
fn main() {
    parse_input();
}

fn parse_input() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut num = 0;
    for line in input.lines() {
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
    println!("{num}");
}

const TESTINPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
