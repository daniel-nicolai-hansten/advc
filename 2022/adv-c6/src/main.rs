use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut hit = false;
    for line in input.lines() {
        for i in 13..line.len() {
            let window = &line[i - 13..=i];
            let mut j = 0;
            let mut dup = false;
            //println!("{}", &window);
            for char in window.chars() {
                if find_char(char, &window[j + 1..]) {
                    dup = true;
                }
                j += 1;
            }
            if !dup {
                println!("marker found at {} - {}", i + 1, &line[i - 13..=i]);
                break;
            }
        }
    }
}
fn find_char(comp: char, s: &str) -> bool {
    let mut ret = false;
    for c in s.chars() {
        if comp == c {
            ret = true;
        }
    }
    ret
}
const TESTINPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
