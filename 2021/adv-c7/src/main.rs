use std::cmp::{min, max};
fn main() {
    // let input = TESTINPUT;
    let input = include_str!("../input.txt");
    let crabs: Vec<_> = input.split(",").map(|x| x.parse::<i32>().unwrap() ).collect();
    let max = crabs.iter().max().unwrap();
    println!("max: {max}");
    let mut min_fuel = i32::MAX;
    for pos in 0..=*max {
        let fuel = crabs.iter().fold(0, |acc, x| acc + (0..=abs(pos, *x)).sum::<i32>() );
        // println!("{fuel} {pos}");
        min_fuel = min(fuel, min_fuel);
        
    }
    println!("{min_fuel}");

}
fn abs(x: i32, y: i32) -> i32 {
    max(x, y) - min(x, y)
}
const TESTINPUT: &str = "16,1,2,0,4,2,7,1,2,14";