use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Brick> {
    let mut ret = vec![];
    for line in input.lines() {
        let line = line.trim();
        let (start, end) = line.split_once("~").unwrap();
        let (sx, sy, sz) = start.split(",").collect_tuple().unwrap();
        let (ex, ey, ez) = end.split(",").collect_tuple().unwrap();
        ret.push((
            (sx.parse().unwrap(), sy.parse().unwrap(), sz.parse().unwrap()),
            (ex.parse().unwrap(), ey.parse().unwrap(), ez.parse().unwrap()),
        ));
    }
    ret
}
type Brick = ((usize, usize, usize), (usize, usize, usize));
#[aoc(day22, part1)]
fn part1(input: &[Brick]) -> u32 {
    println!("{:?}", input);
    let mut final_brick_positions = vec![];
    let mut map = vec![vec![vec![false; 1000]; 1000]; 1000];
    for brick in input {
        let ((sx, sy, mut sz), (ex, ey, mut ez)) = brick;

        'fallloop: loop {
            // check if brick can fall down
            for x in *sx..=*ex {
                for y in *sy..=*ey {
                    if map[x][y][sz - 1] || map[x][y][ez - 1] || ez == 1 || sz == 1 {
                        // can't fall down
                        break 'fallloop;
                    }
                }
            }
            sz -= 1;
            ez -= 1;
        }
        println!("fallen brick: {:?} {:?}", (sx, sy, sz), (ex, ey, ez));
        final_brick_positions.push(((*sx, *sy, sz), (*ex, *ey, ez)));
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in sz..=ez {
                    map[x][y][z] = true;
                }
            }
        }
    }
    //print map from side
    // let mut hasbrick = false;
    // for z in (0..10).rev() {
    //     for y in 0..10 {
    //         for x in 0..10 {
    //             hasbrick = hasbrick || map[x][y][z]
    //         }
    //         print!("{}", if hasbrick { "#" } else { "." });
    //         hasbrick = false;
    //     }
    //     println!();
    // }
    let mut bricks_disintigrate = 0;
    for brick in &final_brick_positions {
        let ((sx, sy, sz), (ex, ey, ez)) = brick;
        for x in (*sx)..=(*ex) {
            for y in (*sy)..=(*ey) {
                for z in (*sz)..=(*ez) {
                    map[x][y][z] = false;
                }
            }
        }
        let mut numfall = 0;
        'outer: for brick2 in final_brick_positions.iter().filter(|b| *b != brick) {
            let mut canfall = true;
            let ((sx, sy, sz), (ex, ey, ez)) = brick2;
            for x in (*sx)..=(*ex) {
                for y in (*sy)..=(*ey) {
                    for z in (*sz)..=(*ez) {
                        map[x][y][z] = false;
                    }
                }
            }
            for x in (*sx)..=(*ex) {
                for y in (*sy)..=(*ey) {
                    if map[x][y][*sz - 1] || map[x][y][*ez - 1] || *ez == 1 || *sz == 1 {
                        // can't fall down
                        canfall = false;
                    }
                }
            }
            if canfall {
                numfall += 1;
            }
            for x in (*sx)..=(*ex) {
                for y in (*sy)..=(*ey) {
                    for z in (*sz)..=(*ez) {
                        map[x][y][z] = true;
                    }
                }
            }
        }
        println!("numfall: {}", numfall);
        if numfall == 0 {
            bricks_disintigrate += 1;
        }
        for x in (*sx)..=(*ex) {
            for y in (*sy)..=(*ey) {
                for z in (*sz)..=(*ez) {
                    map[x][y][z] = true;
                }
            }
        }
    }
    bricks_disintigrate
}

#[aoc(day22, part2)]
fn part2(_input: &[Brick]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 5);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
