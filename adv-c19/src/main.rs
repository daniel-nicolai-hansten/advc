// use nom::lib::std::str::pattern::SearchStep::Done;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};
#[macro_use]
extern crate nom;

fn main() {
    println!("Hello, world!");
}



// fn parse_input(input: &str) -> Vec<GasDir> {
//     let mut ret = vec![];
//     for line in input.lines() {

//     }
//     ret
// }

struct Minerals {
    Ore: usize,
    Clay: usize,
    Obsidian: usize,
}
struct RobotBlueprint {
    OreRobotCost: Minerals,
    ClayRobotCost: Minerals,
    ObsidianRobotCost: Minerals,
    GeodeRobotCost: Minerals,
}
impl RobotBlueprint {


}


const TESTINPUT: &str = 
"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
named!(orerobot, tag_s!("Each ore robot costs"));
#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
         
    }
}