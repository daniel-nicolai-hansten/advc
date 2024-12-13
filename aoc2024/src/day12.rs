use crate::ipos::{Dir, ICoord, IPos};
// use crate::pos::{Coord, Pos};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day12, part1)]
fn part1(map: &[Vec<char>]) -> usize {
    let mut res = 0;
    let mut visited = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _cell) in row.iter().enumerate() {
            let curpos = (x as isize, y as isize);
            if visited.contains(&curpos) {
                continue;
            }
            let (area, perimiter, _) = find_area(map, curpos, &mut visited);
            res += area * perimiter;
        }
    }
    res
}

fn find_area(map: &[Vec<char>], pos: IPos, visited: &mut HashSet<IPos>) -> (usize, usize, Vec<(IPos, Dir)>) {
    let mut que = VecDeque::new();
    que.push_back(pos);
    let id = mappos(map, pos).unwrap();
    let mut area = 0;
    let mut perimiter = 0;
    let mut perimiter_map = Vec::new();

    while let Some(curpos) = que.pop_front() {
        if visited.contains(&curpos) {
            continue;
        }
        visited.insert(curpos);
        area += 1;
        for nextpos in curpos.all_neighbors(map.len() as isize + 2, map[0].len() as isize + 2) {
            match mappos(map, nextpos) {
                Some(mp_id) if mp_id == id => {
                    que.push_back(nextpos);
                }
                _ => {
                    perimiter += 1;
                    for dir in Dir::dirs() {
                        match nextpos.dir(dir) {
                            Some(crps) if crps == curpos => {
                                perimiter_map.push((nextpos, dir));
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
    (area, perimiter, perimiter_map)
}
fn mappos(map: &[Vec<char>], pos: IPos) -> Option<char> {
    let x = usize::try_from(pos.x()).ok()?;
    let y = usize::try_from(pos.y()).ok()?;
    map.get(y)?.get(x).copied()
}

fn find_sides(_map: &[Vec<char>], _pos: IPos, perimiter_map: &mut Vec<(IPos, Dir)>) -> usize {
    let mut sides = 0;
    while !perimiter_map.is_empty() {
        let (startpos, startdir) = perimiter_map.iter().find_or_first(|(_p, _d)| true).unwrap().clone();
        let mut que = VecDeque::new();
        que.push_back(startpos);
        while let Some(pos) = que.pop_front() {
            let (pos_idx, _) = perimiter_map.iter().find_position(|(p, d)| &pos == p && startdir == *d).unwrap();
            for pis in pos
                .neighbors(-1, isize::MAX, -1, isize::MAX)
                .iter()
                .filter(|&p| perimiter_map.contains(&(*p, startdir)))
            {
                que.push_back(*pis);
            }
            let _ = perimiter_map.swap_remove(pos_idx);
        }
        sides += 1;
    }
    sides
}
#[aoc(day12, part2)]
fn part2(map: &[Vec<char>]) -> usize {
    let mut res = 0;
    let mut visited = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _cell) in row.iter().enumerate() {
            let curpos: IPos = (x as isize, y as isize);
            if visited.contains(&curpos) {
                continue;
            }
            let (area, _perimiter, mut perimiter_map) = find_area(map, curpos, &mut visited);
            let sides = find_sides(map, curpos, &mut perimiter_map);
            res += area * sides;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const TESTINPUT2: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const TESTINPUT3: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    #[test]
    fn part1_example() {
        let map = parse(TESTINPUT);
        let mut visited = HashSet::new();
        let (area, perimiter, _) = find_area(&map, (0, 0), &mut visited);
        assert_eq!(area, 12);
        assert_eq!(perimiter, 18);
        assert_eq!(part1(&parse(TESTINPUT)), 1930);
    }

    #[test]
    fn part2_example() {
        let map = parse(TESTINPUT2);
        let mut visited = HashSet::new();
        let (area, _perimiter, mut edgemap) = find_area(&map, (0, 0), &mut visited);

        let sides = find_sides(&map, (0, 0), &mut edgemap);
        assert_eq!(sides, 12);
        assert_eq!(area, 17);
        assert_eq!(part2(&parse(TESTINPUT)), 1206);
        assert_eq!(part2(&parse(TESTINPUT3)), 368);
    }
}
