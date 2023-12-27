use std::collections::HashMap;

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
fn part1(input: &[Brick]) -> usize {
    let mut brick_positions = vec![];
    let mut map = vec![vec![vec![false; 1000]; 1000]; 1000];
    let mut brickindex = HashMap::new();
    //draw bricks
    for ((sx, sy, sz), (ex, ey, ez)) in input.iter() {
        brick_positions.push(((*sx, *sy, *sz), (*ex, *ey, *ez)));
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in *sz..=*ez {
                    map[x][y][z] = true;
                }
            }
        }
    }
    //Drop bricks
    loop {
        let mut brick_fallen = false;
        for (_brick_index, brick) in brick_positions.iter_mut().enumerate() {
            let ((sx, sy, sz), (ex, ey,  ez)) = brick;
            for x in *sx..=*ex {
                for y in *sy..=*ey {
                    for z in *sz..=*ez {
                        map[x][y][z] = false;
                    }
                }
            }
            'fallloop: loop {
                // check if brick can fall down
                for x in *sx..=*ex {
                    for y in *sy..=*ey {
                        if map[x][y][*sz - 1] || map[x][y][*ez - 1] || *ez == 1 || *sz == 1 {
                            // can't fall down
                            break 'fallloop;
                        }
                    }
                }
                *sz -= 1;
                *ez -= 1;
                brick_fallen = true;
            }

            for x in *sx..=*ex {
                for y in *sy..=*ey {
                    for z in *sz..=*ez {
                        map[x][y][z] = true;
                    }
                }
            }
        }
        if !brick_fallen {
            break;
        }
    }
    println!("{:?}", brick_positions.len());
for (idx, ((sx, sy, sz), (ex, ey,  ez))) in brick_positions.iter().enumerate() {
    for x in *sx..=*ex {
        for y in *sy..=*ey {
            for z in *sz..=*ez {
                brickindex.insert((x,y,z), idx);
            }
        }
    }
}
    let mut single_list = vec![0; brick_positions.len()];
    for (idx, brick) in brick_positions.iter().enumerate() {
        let mut bricks_under = vec![];
        let ((sx, sy, sz), (ex, ey, ez)) = brick;
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in *sz..=*ez {
                    let block_over = (x, y, z - 1);
                    if let Some(brick_idx) = brickindex.get(&block_over) {
                        if *brick_idx != idx {
                            bricks_under.push(*brick_idx);
                        }
                    }
                }
            }
        }
        println!("idx: {}  resting on: {:?}, brick {:?}", idx, bricks_under, brick);
        if bricks_under.iter().unique().count() == 1 {
            for b in bricks_under {
                let ptr = single_list.get_mut(b).unwrap();
                *ptr += 1;
            }
        }

    }

    let bricks_disintigrate = single_list.iter().filter(|i| **i == 0).count();
    bricks_disintigrate
}

#[aoc(day22, part2)]
fn part2(input: &[Brick]) -> usize {
    let mut brick_positions = vec![];
    let mut map = vec![vec![vec![false; 1000]; 1000]; 1000];
    let mut brickindex = HashMap::new();
    //draw bricks
    for ((sx, sy, sz), (ex, ey, ez)) in input.iter() {
        brick_positions.push(((*sx, *sy, *sz), (*ex, *ey, *ez)));
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in *sz..=*ez {
                    map[x][y][z] = true;
                }
            }
        }
    }
    //Drop bricks
    loop {
        let mut brick_fallen = false;
        for (_brick_index, brick) in brick_positions.iter_mut().enumerate() {
            let ((sx, sy, sz), (ex, ey,  ez)) = brick;
            for x in *sx..=*ex {
                for y in *sy..=*ey {
                    for z in *sz..=*ez {
                        map[x][y][z] = false;
                    }
                }
            }
            'fallloop: loop {
                // check if brick can fall down
                for x in *sx..=*ex {
                    for y in *sy..=*ey {
                        if map[x][y][*sz - 1] || map[x][y][*ez - 1] || *ez == 1 || *sz == 1 {
                            // can't fall down
                            break 'fallloop;
                        }
                    }
                }
                *sz -= 1;
                *ez -= 1;
                brick_fallen = true;
            }

            for x in *sx..=*ex {
                for y in *sy..=*ey {
                    for z in *sz..=*ez {
                        map[x][y][z] = true;
                    }
                }
            }
        }
        if !brick_fallen {
            break;
        }
    }
    println!("{:?}", brick_positions.len());
for (idx, ((sx, sy, sz), (ex, ey,  ez))) in brick_positions.iter().enumerate() {
    for x in *sx..=*ex {
        for y in *sy..=*ey {
            for z in *sz..=*ez {
                brickindex.insert((x,y,z), idx);
            }
        }
    }
}
    let mut single_list = vec![0; brick_positions.len()];
    for (idx, brick) in brick_positions.iter().enumerate() {
        let mut bricks_under = vec![];
        let ((sx, sy, sz), (ex, ey, ez)) = brick;
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in *sz..=*ez {
                    let block_over = (x, y, z - 1);
                    if let Some(brick_idx) = brickindex.get(&block_over) {
                        if *brick_idx != idx {
                            bricks_under.push(*brick_idx);
                        }
                    }
                }
            }
        }
        println!("idx: {}  resting on: {:?}, brick {:?}", idx, bricks_under, brick);
        if bricks_under.iter().unique().count() == 1 {
            for b in bricks_under {
                let ptr = single_list.get_mut(b).unwrap();
                *ptr += 1;
            }
        }

    }

    let bricks_disintigrate = single_list.iter().filter(|i| **i == 0).count();
    bricks_disintigrate
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
