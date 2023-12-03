use std::fs;

fn main() {
    // let mut fishies: Vec<(usize, u8)> = TESTINPUT
    //     .split(",")
    //     .map(|x| (1, x.parse::<u8>().unwrap()))
    //     .collect();
    let mut fishies: Vec<(usize, u8)> = fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|x| (1, x.parse::<u8>().unwrap()))
        .collect();
    for i in 0..256 {
        println!("{}: {}", i, fishies.len());
        let mut new_fish = 0;
        for (nums, fish) in fishies.iter_mut() {
            if *fish > 0 {
                *fish -= 1;
            } else {
                *fish = 6;
                new_fish += *nums;
            }
        }

        if new_fish > 0 {
            fishies.push((new_fish, 8));
        }
    }
    let mut tot = 0;
    for (nums, _fish) in fishies {
        tot += nums;
    }

    println!("{}", tot);
}
const TESTINPUT: &str = "3,4,3,1,2";
