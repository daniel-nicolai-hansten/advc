use std::collections::VecDeque;
use num_bigint::BigUint;
#[allow(unused_variables)]
use std::fs;
const NUM: usize = 4;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut monkeys = [
        Monkey::new(),
        Monkey::new(),
        Monkey::new(),
        Monkey::new(),
        Monkey::new(),
        Monkey::new(),
        Monkey::new(),
        Monkey::new(),
    ];
    let mut currentmonkey = 0;
    for line in input.lines() {
        if line.len() >= 6 {
            let stripped_line = &line.trim();
            //println!("{}", &stripped_line[..6]);
            match &stripped_line[..6] {
                "Monkey" => {
                    //println!("{}", &stripped_line[7..]);
                    currentmonkey = stripped_line[7..]
                        .strip_suffix(":")
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
                "Starti" => {
                    let itemlist: Vec<&str> = stripped_line[15..].split(",").collect();
                    for item in itemlist {
                        monkeys[currentmonkey]
                            .items
                            .push_back(item.trim().parse::<u128>().unwrap());
                        //println!("{}", item.trim());
                    }
                }
                "Operat" => match &stripped_line[17..22] {
                    "old *" => {
                        //println!("{}", &stripped_line[17..23]);
                        if &stripped_line[17..] == "old * old" {
                            monkeys[currentmonkey].operation = MonkeyOperation::OldMultiplyOld;
                        } else {
                            monkeys[currentmonkey].operation = MonkeyOperation::OldMultiply;
                            monkeys[currentmonkey].operation_num =
                                stripped_line[23..].parse::<u32>().unwrap();
                        }
                    }
                    "old +" => {
                        monkeys[currentmonkey].operation = MonkeyOperation::OldPlus;
                        monkeys[currentmonkey].operation_num =
                            stripped_line[23..].parse::<u32>().unwrap();
                    }
                    _ => {
                        println!("{}", &stripped_line);
                    }
                },
                "Test: " => {
                    monkeys[currentmonkey].test_div = stripped_line[19..].parse::<u32>().unwrap();
                }
                "If tru" => {
                    monkeys[currentmonkey].monkey_t = stripped_line[25..].parse::<usize>().unwrap();
                }
                "If fal" => {
                    monkeys[currentmonkey].monkey_f = stripped_line[26..].parse::<usize>().unwrap();
                }
                &_ => {}
            }
        }
    }
    let mod_product = monkeys.iter().map(|x| x.test_div).product::<u32>();
    for cycle in 0..10000 {
        println!("{}",cycle);
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop_front().unwrap();
                //println!("item {}", item);
                monkeys[i].items_inspected += 1;
                match &monkeys[i].operation {
                    MonkeyOperation::OldPlus => item = item + monkeys[i].operation_num as u128,
                    MonkeyOperation::OldMultiply => item = item * monkeys[i].operation_num as u128,
                    MonkeyOperation::OldMultiplyOld => item = item * item,
                }
                //item = item % mod_product as u128;
                if item % monkeys[i].test_div as u128 == 0 {
                    monkeys[monkeys[i].monkey_t].items.push_back(item);
                } else {
                    monkeys[monkeys[i].monkey_f].items.push_back(item);
                }
            }
        }
    }
    let mut results = vec![];
    for i in 0..monkeys.len() {
        results.push(monkeys[i].items_inspected);
        println!("inspected {}", monkeys[i].items_inspected);
    }
    results.sort();
    println!("{} {} {}", &results[6], &results[7] , &results[6] * &results[7]);
}
#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    operation: MonkeyOperation,
    operation_num: u32,
    test_div: u32,
    monkey_f: usize,
    monkey_t: usize,
    items_inspected: usize,
}
impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            operation: MonkeyOperation::OldPlus,
            operation_num: 0,
            test_div: 0,
            monkey_f: 0,
            monkey_t: 0,
            items_inspected: 0,
        }
    }
}
#[derive(Debug)]
enum MonkeyOperation {
    OldPlus,
    OldMultiplyOld,
    OldMultiply,
}

const TESTINPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1
";
