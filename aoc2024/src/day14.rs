use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    sequence::{preceded, separated_pair},
    IResult,
};

use std::{error::Error, fs::File};
use std::{io::BufWriter, sync::mpsc, thread};
#[cfg(test)]
const MAX_W: usize = 11;
#[cfg(test)]
const MAX_H: usize = 7;

#[cfg(not(test))]
const MAX_W: usize = 101;
#[cfg(not(test))]
const MAX_H: usize = 103;
#[derive(Debug, Clone)]
struct Robot {
    position: (usize, usize),
    velocity: (i32, i32),
}
impl Robot {
    fn run(&mut self) {
        let new_x = (self.position.0 as i32).wrapping_add(self.velocity.0);
        let new_y = (self.position.1 as i32).wrapping_add(self.velocity.1);
        self.position.0 = ((new_x % MAX_W as i32 + MAX_W as i32) % MAX_W as i32) as usize;
        self.position.1 = ((new_y % MAX_H as i32 + MAX_H as i32) % MAX_H as i32) as usize;
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
    Ok((
        i,
        Robot {
            position: (o.0 .0 as usize, o.0 .1 as usize),
            velocity: o.1,
        },
    ))
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

    quadrant1 * quadrant2 * quadrant3 * quadrant4
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> usize {
    let mut ret = 0;
    let (sender, reciver) = mpsc::channel();
    let mut robots = input.to_vec();
    let joinhandle = thread::spawn(move || {
        for _i in 0..10000 {
            for r in robots.iter_mut() {
                r.run();
            }
            let mut img = vec![vec![0_u8; MAX_W]; MAX_H];
            for r in robots.iter() {
                img[r.position.1][r.position.0] = 0xff;
            }
            let _ = sender.send(img);
        }
    });
    let mut imgs_checked = 0;
    while let Ok(img) = reciver.recv() {
        imgs_checked += 1;
        let cnt = med_filter(&img);
        if cnt > 200 {
            // println!("something detected at img: {imgs_checked} cnt: {cnt}");
            ret = imgs_checked;
            break;
        }
    }
    let _ = joinhandle.join();
    ret
}
const MED_IDX: usize = 3;
fn med_filter(img: &[Vec<u8>]) -> usize {
    let mut ret = 0;
    let neigh = |x: usize, y: usize| {
        if y == 0 || x == 0 {
            return None;
        }
        let mut res: usize = img.get(y - 1)?.get(x - 1..=x + 1)?.iter().fold(0, |acc, px| acc + *px as usize);
        res += img.get(y)?.get(x - 1..=x + 1)?.iter().fold(0, |acc, px| acc + *px as usize);
        res += img.get(y + 1)?.get(x - 1..=x + 1)?.iter().fold(0, |acc, px| acc + *px as usize);
        Some(res > MED_IDX * 0xff)
    };
    for (y, ln) in img.iter().enumerate() {
        for (x, _px) in ln.iter().enumerate() {
            match neigh(x, y) {
                Some(true) => ret += 1,
                _ => (),
            }
        }
    }
    ret
}

#[allow(dead_code)]
fn robots_to_png(robots: &[Robot], num: usize) -> Result<(), Box<dyn Error>> {
    let mut img = vec![0; MAX_W * MAX_H];
    let path = format!("/tmp/{num}.png");
    for r in robots.iter() {
        img[(r.position.1 * MAX_W + r.position.0) as usize] = 255;
    }
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, MAX_W as u32, MAX_H as u32); // Width is 2 pixels and height is 1.
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
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 12);
    }
}
