use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Brick> {
    let mut ret = vec![];
    let mut map = vec![vec![vec![false; 350]; 40]; 40];
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
    let mut brick_positions = vec![];

    //draw bricks
    for ((sx, sy, sz), (ex, ey, ez)) in ret.iter() {
        brick_positions.push(((*sx, *sy, *sz), (*ex, *ey, *ez)));
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in *sz..=*ez {
                    map[x as usize][y as usize][z] = true;
                }
            }
        }
    }
    //Drop bricks
    loop {
        let mut brick_fallen = false;
        for (_brick_index, brick) in brick_positions.iter_mut().enumerate() {
            let ((sx, sy, sz), (ex, ey, ez)) = brick;
            for x in *sx..=*ex {
                for y in *sy..=*ey {
                    for z in *sz..=*ez {
                        map[x as usize][y as usize][z] = false;
                    }
                }
            }
            'fallloop: loop {
                // check if brick can fall down
                for x in *sx..=*ex {
                    for y in *sy..=*ey {
                        if map[x as usize][y as usize][*sz - 1]
                            || map[x as usize][y as usize][*ez - 1]
                            || *ez == 1
                            || *sz == 1
                        {
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
                        map[x as usize][y as usize][z] = true;
                    }
                }
            }
        }
        if !brick_fallen {
            break;
        }
    }
    brick_positions.sort_unstable_by_key(|(_, (_, _, z))| *z);
    brick_positions
}
type Brick = ((usize, usize, usize), (usize, usize, usize));
#[aoc(day22, part1)]
fn part1(brick_positions: &[Brick]) -> usize {
    let mut brickindex = HashMap::default();

    for (idx, ((sx, sy, sz), (ex, ey, ez))) in brick_positions.iter().enumerate() {
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in *sz..=*ez {
                    brickindex.insert((x, y, z), idx);
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
        // println!("idx: {}  resting on: {:?}, brick {:?}", idx, bricks_under, brick);
        if bricks_under.iter().unique().count() == 1 {
            for b in bricks_under {
                let ptr = single_list.get_mut(b).unwrap();
                *ptr += 1;
            }
        }
    }

    single_list.iter().filter(|i| **i == 0).count()
}

#[aoc(day22, part2)]
fn part2(brick_positions: &[Brick]) -> usize {
    let mut brickindex = HashMap::default();
    for (idx, ((sx, sy, sz), (ex, ey, ez))) in brick_positions.iter().enumerate() {
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in *sz..=*ez {
                    brickindex.insert((x, y, z), idx);
                }
            }
        }
    }
    let mut brick_tree = vec![];
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
        // println!("idx: {}  resting on: {:?}, brick {:?}", idx, bricks_under, brick);
        brick_tree.push(bricks_under);
    }

    (0..brick_tree.len())
        .into_par_iter()
        .map(|idx| {
            let mut bricks_falling: Vec<usize> = vec![idx];
            loop {
                for (b_idx, brick) in brick_tree.iter().enumerate() {
                    match (
                        brick.len(),
                        brick.iter().filter(|brk| !bricks_falling.contains(brk)).count(),
                    ) {
                        (0, _) => (),
                        (_, 0) if !bricks_falling.contains(&b_idx) => {
                            bricks_falling.push(b_idx);
                        }
                        (_, _) => (),
                    }
                }
                break;
            }
            bricks_falling.len() - 1
        })
        .sum()
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

    // #[test]
    // fn part1_example() {
    //     assert_eq!(part1(&parse(TESTINPUT)), 5);
    // }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 7);
    }
}
