use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    sequence::{preceded, separated_pair},
    IResult,
};

use std::io::BufWriter;
use std::{error::Error, fs::File};
use opencv::prelude::*;

const MAX_W: u32 = 101;
const MAX_H: u32 = 103;
#[derive(Debug, Clone)]
struct Robot {
    position: (u32, u32),
    velocity: (i32, i32),
}
impl Robot {
    fn run(&mut self) {
        let new_x = (self.position.0 as i32).wrapping_add(self.velocity.0);
        let new_y = (self.position.1 as i32).wrapping_add(self.velocity.1);

        // Wrap around using modulo, handling negative numbers
        self.position.0 = ((new_x % MAX_W as i32 + MAX_W as i32) % MAX_W as i32) as u32;
        self.position.1 = ((new_y % MAX_H as i32 + MAX_H as i32) % MAX_H as i32) as u32;
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(|l| parse_line(l).unwrap().1).collect()
}

fn parse_line(line: &str) -> IResult<&str, Robot> {
    let (i, o) = separated_pair(
        preceded(tag("p="), separated_pair(complete::u32, tag(","), complete::u32)),
        space1,
        preceded(tag("v="), separated_pair(complete::i32, tag(","), complete::i32)),
    )(line)?;
    Ok((i, Robot { position: o.0, velocity: o.1 }))
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> usize {
    let mut robots = input.to_vec();
    for _ in 0..100 {
        for r in robots.iter_mut() {
            r.run();
        }
    }

    let quadrant1 = robots.iter().filter(|r| r.position.0 < MAX_W / 2 && r.position.1 < MAX_H / 2).count();
    let quadrant2 = robots.iter().filter(|r| r.position.0 > MAX_W / 2 && r.position.1 < MAX_H / 2).count();
    let quadrant3 = robots.iter().filter(|r| r.position.0 < MAX_W / 2 && r.position.1 > MAX_H / 2).count();
    let quadrant4 = robots.iter().filter(|r| r.position.0 > MAX_W / 2 && r.position.1 > MAX_H / 2).count();
    println!("{} {} {} {} {}", quadrant1, quadrant2, quadrant3, quadrant4, robots.len());

    quadrant1 * quadrant2 * quadrant3 * quadrant4
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> usize {
    let mut robots = input.to_vec();
    for i in 0..10000 {
        for r in robots.iter_mut() {
            r.run();
        }

        robots_to_png(&robots, i).unwrap();
    }
    0
}
fn robots_to_png(robots: &[Robot], num: usize) -> Result<(), Box<dyn Error>> {
    let mut img = vec![0; MAX_W as usize * MAX_H as usize];
    let path = format!("/tmp/{num}.png");
    for r in robots.iter() {
        img[(r.position.1 * MAX_W + r.position.0) as usize] = 255;
    }
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, MAX_W, MAX_H); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&img).unwrap(); // Save

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    // #[test]
    // fn part1_example() {
    //     let mut robots = parse("p=2,4 v=2,-3");
    //     for _ in 0..4 {
    //         println!("{:?}", robots);
    //         for y in 0..MAX_H {
    //             for x in 0..MAX_W {
    //                 if robots.iter().any(|r| r.position == (x, y)) {
    //                     print!("#");
    //                 } else {
    //                     print!(".");
    //                 }
    //             }
    //             println!();
    //         }
    //         robots[0].run();
    //     }
    //     let half1 = MAX_W / 2;
    //     println!("{} {}", half1, MAX_W);
    //     assert_eq!(part1(&parse(TESTINPUT)), 12);
    // }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 12);
    }
}
