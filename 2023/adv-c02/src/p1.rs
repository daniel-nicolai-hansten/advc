use std::fs;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut gamesum = 0;
    for line in input.lines() {
        let mut possible = true;
        let splits: Vec<&str> = line.split(": ").collect();
        let gamenum: Vec<&str> = splits[0].split(" ").collect();
        let substring = splits[1];
        for subsets in substring.split("; ") {
            let (mut r, mut g, mut b) = (0, 0, 0);
            for colour in subsets.split(", ") {
                let c: Vec<&str> = colour.split(" ").collect();
                match c[1] {
                    "red" => r = i32::from_str_radix(c[0], 10).unwrap(),
                    "green" => g = i32::from_str_radix(c[0], 10).unwrap(),
                    "blue" => b = i32::from_str_radix(c[0], 10).unwrap(),
                    _ => (),
                }
            }
            println!("r:{r} g:{g} b:{b}");
            if r > 12 || g > 13 || b > 14 {
                possible = false;
            }
        }
        if possible {
            gamesum += i32::from_str_radix(gamenum[1], 10).unwrap();
        }
    }
    println!("{gamesum}");
}
const TESTINPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
