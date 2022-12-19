use std::{collections::VecDeque, fs};

// use nom::lib::std::str::pattern::SearchStep::Done;
use nom::{
    bytes::complete::tag,
    character::complete,
    error::context,
    sequence::{delimited, separated_pair},
    *,
};
#[derive(Debug)]
struct ObsidianRequirements {
    ore: usize,
    clay: usize,
}
#[derive(Debug)]
struct GeodeRequirements {
    ore: usize,
    obsidian: usize,
}
//#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: ObsidianRequirements,
    geode: GeodeRequirements,
}
impl Blueprint {
    fn calculate_maxprices(&self) -> Minerals {
        let mut ret = Minerals::new();
        ret.obsidian = self.geode.obsidian;
        ret.clay = self.obsidian.clay;
        ret.ore = self.ore;
        if self.clay > ret.ore {
            ret.ore = self.clay;
        }
        if self.obsidian.ore > ret.ore {
            ret.ore = self.obsidian.ore;
        }
        if self.geode.ore > ret.ore {
            ret.ore = self.geode.ore;
        }
        ret
    }
}
#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq)]
struct Minerals {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}
impl Minerals {
    fn new() -> Self {
        Minerals {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}
#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq)]
struct Robots {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}
impl Robots {
    fn new() -> Self {
        Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut tot = vec![];
    let mut i = 0;
    for line in input.lines() {
        i += 1;
        if i <= 3 {
            let (_, blueprint1) = blueprint(line).unwrap();
            tot.push(run_simulation(&blueprint1));
        }
    }
    let tot2 = tot[0] * tot[1] * tot[2];
    println!("{}", tot2);
}

fn buy_robot(
    blueprint: &Blueprint,
    robot: &RobotType,
    resources: &mut Minerals,
    robots: &mut Robots,
) {
    match robot {
        RobotType::Ore => {
            resources.ore -= blueprint.ore;
            robots.ore += 1;
        }
        RobotType::Clay => {
            resources.ore -= blueprint.clay;
            robots.clay += 1;
        }
        RobotType::Obsidian => {
            resources.clay -= blueprint.obsidian.clay;
            resources.ore -= blueprint.obsidian.ore;
            robots.obsidian += 1;
        }
        RobotType::Geode => {
            resources.ore -= blueprint.geode.ore;
            resources.obsidian -= blueprint.geode.obsidian;
            robots.geode += 1;
        }
    }
}
#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq)]
struct SimulationContext {
    robots: Robots,
    resorces: Minerals,
    nextmove: Option<RobotType>,
}
impl SimulationContext {
    fn calculate_value(&self, prices: &Minerals) -> usize {
        let mut res = self.resorces.ore + self.robots.ore;
        res += (self.resorces.clay + self.robots.clay) * prices.ore;
        res += (self.resorces.obsidian + self.robots.obsidian) * prices.ore * prices.clay;
        res +=
            (self.resorces.geode + self.robots.geode) * prices.ore * prices.clay * prices.obsidian;

        res
    }
}
fn run_simulation(blueprint: &Blueprint) -> usize {
    let mut timer = 0;
    let mut max_geodes = 0;
    let max_prices = blueprint.calculate_maxprices();
    let mut workque = VecDeque::new();
    workque.push_back((
        0,
        SimulationContext {
            robots: Robots::new(),
            resorces: Minerals::new(),
            nextmove: None,
        },
    ));
    while timer < 32 {
        timer += 1;
        //println!("{}", timer);
        if workque.len() > 900 {
            workque.make_contiguous().sort_by(|a, b| b.cmp(a));
            workque.drain(900..);
        }
        for _ in 0..workque.len() {
            let (_, mut context) = workque.pop_front().unwrap();
            //println!("{:?} {:?}", context, max_prices);
            context.resorces.ore += context.robots.ore;
            context.resorces.clay += context.robots.clay;
            context.resorces.obsidian += context.robots.obsidian;
            context.resorces.geode += context.robots.geode;
            if let Some(nextmove) = context.nextmove {
                //println!("bying {:?}", nextmove);
                buy_robot(
                    blueprint,
                    &nextmove,
                    &mut context.resorces,
                    &mut context.robots,
                );
            }
            if max_geodes < context.resorces.geode {
                max_geodes = context.resorces.geode
            }
            for validmove in
                find_next_valid_move(blueprint, &max_prices, &context.resorces, &context.robots)
            {
                context.nextmove = validmove;
                workque.push_back((context.calculate_value(&max_prices), context.clone()));
            }
        }
    }
    println!("geodes: {}", max_geodes);
    max_geodes
}
fn find_next_valid_move(
    blueprint: &Blueprint,
    max_prices: &Minerals,
    resorces: &Minerals,
    robots: &Robots,
) -> Vec<Option<RobotType>> {
    let mut res = vec![];
    if blueprint.geode.ore <= resorces.ore && blueprint.geode.obsidian <= resorces.obsidian {
        res.push(Some(RobotType::Geode));
    } else {
        res.push(None);

        if max_prices.obsidian > robots.obsidian
            && blueprint.obsidian.ore <= resorces.ore
            && blueprint.obsidian.clay <= resorces.clay
        {
            res.push(Some(RobotType::Obsidian));
        }
        if max_prices.clay > robots.clay && blueprint.clay <= resorces.ore {
            res.push(Some(RobotType::Clay));
        }
        if max_prices.ore > robots.ore && blueprint.ore <= resorces.ore {
            res.push(Some(RobotType::Ore));
        }
    }
    res
}
fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), complete::u64, tag(":"))(input)?;
    let (input, ore) =
        delimited(tag(" Each ore robot costs "), complete::u64, tag(" ore."))(input)?;
    let (input, clay) =
        delimited(tag(" Each clay robot costs "), complete::u64, tag(" ore."))(input)?;
    let (input, obsidian) = delimited(
        tag(" Each obsidian robot costs "),
        separated_pair(complete::u64, tag(" ore and "), complete::u64).map(|pair| {
            ObsidianRequirements {
                ore: pair.0 as usize,
                clay: pair.1 as usize,
            }
        }),
        tag(" clay."),
    )(input)?;
    let (input, geode) = delimited(
        tag(" Each geode robot costs "),
        separated_pair(complete::u64, tag(" ore and "), complete::u64).map(|pair| {
            GeodeRequirements {
                ore: pair.0 as usize,
                obsidian: pair.1 as usize,
            }
        }),
        tag(" obsidian."),
    )(input)?;
    Ok((
        input,
        Blueprint {
            id: id as usize,
            ore: ore as usize,
            clay: clay as usize,
            obsidian,
            geode,
        },
    ))
}

const TESTINPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
