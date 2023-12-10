use std::fs;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut fishies: Vec<u8> = TESTINPUT
        .split(",")
        .map(|x|  x.parse::<u8>().unwrap())
        .collect();
    // let mut fishies: Vec< u8> = fs::read_to_string("input.txt")
    //     .unwrap()
    //     .split(",")
    //     .map(|x| x.parse::<u8>().unwrap())
    //     .collect();
    for i in 0..=256 {
        println!("{}: {}", i, fishies.len());
        let mut new_fish = 0;
        for fish in fishies.iter_mut() {
            if *fish > 0 {
                *fish -= 1;
            } else {
                *fish = 6;
                new_fish += 1;
            }
        }

        if new_fish > 0 {
            for _ in 0..new_fish {
                fishies.push(8);
            }
        }
    }
    let tot = fishies.len();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{}", tot);
}
const TESTINPUT: &str = "3,4,3,1,2";
