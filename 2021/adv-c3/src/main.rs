use std::{fs, isize};
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut counters = [0; 5];
    let mut nums = [0; 20];
    let mut j = 0;
    for line in TESTINPUT.lines() {
        let mut i = 0;
        for c in line.chars() {
            match c {
                '0' => counters[i] -= 1,
                '1' => counters[i] += 1,
                _ => {}
            }
            i += 1;
        }
        nums[j] = isize::from_str_radix(line, 2).unwrap();
        j += 1;
    }

    println!("{:?}", counters);
    //let filter =
    let mut filternum = 0;
    {
        let mut bitpos = 4;
        for c in counters {
            if c > 0 {
                filternum |= 1 << bitpos;
            }
            bitpos -= 1;
        }
        println!("filternum {:b}", filternum);
    }
    for i in 0..4 {
        for num in nums {
            let filter = filternum | 0x1f << 5 - i;
            if filter | num != 0 {
                println!("filter {:b}", filter);
            }
            println!("{}", num);
        }
    }
}

const TESTINPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
