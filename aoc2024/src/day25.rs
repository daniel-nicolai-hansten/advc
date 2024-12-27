use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
fn parse(input: &str) -> Vec<([u32; 5], Type)> {
    let mut ret = vec![];
    let mut grid = [0; 5];
    let mut schem_type = Type::Lock;
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            ret.push((grid, schem_type));
            grid = [0; 5];
            schem_type = Type::Lock;
        }
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => grid[x] += 1,
                '.' => (),
                _ => unreachable!(),
            }
        }
        if y % 8 == 0 && grid.iter().sum::<u32>() == 0 {
            schem_type = Type::Key;
        }
    }
    ret.push((grid, schem_type));
    ret
}

#[derive(Debug)]
enum Type {
    Key,
    Lock,
}
#[aoc(day25, part1)]
fn part1(input: &Vec<([u32; 5], Type)>) -> u32 {
    let mut res = 0;
    let keys = input.iter().filter(|(_, t)| matches!(t, Type::Key)).collect::<Vec<_>>();
    let locks = input.iter().filter(|(_, t)| matches!(t, Type::Lock)).collect::<Vec<_>>();
    for (key, _) in keys {
        'lock: for (lock, _) in &locks {
            for (idx, pin) in lock.iter().enumerate() {
                if key[idx] + *pin > 7 {
                    continue 'lock;
                }
            }
            res += 1;
        }
    }
    res
}

#[aoc(day25, part2)]
fn part2(_input: &Vec<([u32; 5], Type)>) -> String {
    "N/A".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 3);
    }

    #[test]
    fn part2_example() {
        // assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
