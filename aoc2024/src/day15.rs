use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn next(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        }
    }
}
#[aoc_generator(day15)]
fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Dir>) {
    let (map, dirs) = separated_pair(parse_map, many1(newline), parse_dirs)(input).unwrap().1;
    (map, dirs)
}
fn parse_map(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (i, o) = separated_list1(newline, many1(one_of("#.O@")))(input)?;
    Ok((i, o))
}
fn parse_dirs(input: &str) -> IResult<&str, Vec<Dir>> {
    let (i, o) = separated_list1(
        newline,
        many1(one_of("^v<>").map(|o| match o {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => unreachable!(),
        })),
    )(input)?;
    let o = o.into_iter().flatten().collect();
    Ok((i, o))
}

#[aoc(day15, part1)]
fn part1(input: &(Vec<Vec<char>>, Vec<Dir>)) -> usize {
    let (map, dirs) = input;
    let mut map = map.clone();
    let mut robotpos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '@').map(|x| (x, y)))
        .unwrap();
    for dir in dirs {
        let (x, y) = robotpos;
        let (nx, ny) = dir.next(x, y);
        match map[ny][nx] {
            '#' => continue,
            '.' => {
                map[y][x] = '.';
                map[ny][nx] = '@';
                robotpos = (nx, ny);
            }
            'O' => {
                if move_box(&mut map, nx, ny, dir) {
                    map[y][x] = '.';
                    map[ny][nx] = '@';
                    robotpos = (nx, ny);
                }
            }
            _ => unreachable!(),
        }
    }
    box_gps(&map)
}
fn box_gps(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &c)| if c == 'O' || c == '[' { Some(100 * y + x) } else { None })
        })
        .sum()
}

fn move_box(map: &mut Vec<Vec<char>>, x: usize, y: usize, dir: &Dir) -> bool {
    let (nx, ny) = dir.next(x, y);
    match map[ny][nx] {
        '#' => false,
        'O' => {
            if move_box(map, nx, ny, dir) {
                map[y][x] = '.';
                map[ny][nx] = 'O';
                true
            } else {
                false
            }
        }
        '.' => {
            map[y][x] = '.';
            map[ny][nx] = 'O';
            true
        }
        _ => false,
    }
}
fn move_box2(map: &[Vec<char>], x: usize, y: usize, dir: &Dir) -> Option<Vec<(usize, usize, char)>> {
    let (you, buddy) = match map[y][x] {
        '[' => ((x, y), (x + 1, y)),
        ']' => ((x - 1, y), (x, y)),
        _ => unreachable!(),
    };
    let (nx, ny) = dir.next(you.0, you.1);
    let (bnx, bny) = dir.next(buddy.0, buddy.1);
    let mut ret = match (map[ny][nx], map[bny][bnx], dir) {
        ('#', _, _) | (_, '#', _) => None,
        ('[', ']', Dir::Up | Dir::Down) => move_box2(map, nx, ny, dir),
        (']', '[', Dir::Up | Dir::Down) => {
            let mut v1 = move_box2(map, nx, ny, dir)?;
            let mut v2 = move_box2(map, bnx, bny, dir)?;
            v1.append(&mut v2);
            Some(v1)
        }
        (']', '.', Dir::Up | Dir::Down) => move_box2(map, nx, ny, dir),
        ('.', '[', Dir::Up | Dir::Down) => move_box2(map, bnx, bny, dir),

        ('.', '.', Dir::Up | Dir::Down) => Some(vec![]),
        ('.', _, Dir::Left) | (_, '.', Dir::Right) => Some(vec![]),
        (']', _, Dir::Left) => move_box2(map, nx, ny, dir),
        (_, '[', Dir::Right) => move_box2(map, bnx, bny, dir),
        _ => None,
    }?;
    ret.push((you.0, you.1, '['));
    ret.push((buddy.0, buddy.1, ']'));
    Some(ret)
}

#[aoc(day15, part2)]
fn part2(input: &(Vec<Vec<char>>, Vec<Dir>)) -> usize {
    let (map, dirs) = input;
    let mut map: Vec<Vec<char>> = map
        .iter()
        .map(|ln| {
            ln.iter()
                .map(|c| match c {
                    '.' => ['.', '.'],
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                })
                .flatten()
                .collect()
        })
        .collect();
    let mut robotpos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '@').map(|x| (x, y)))
        .unwrap();
    for dir in dirs {
        let (x, y) = robotpos;
        let (nx, ny) = dir.next(x, y);
        match map[ny][nx] {
            '#' => continue,
            '.' => {
                map[y][x] = '.';
                map[ny][nx] = '@';
                robotpos = (nx, ny);
            }
            '[' | ']' => {
                if let Some(boxes) = move_box2(&map, nx, ny, dir) {
                    for bx in &boxes {
                        map[bx.1][bx.0] = '.';
                    }
                    for bx in boxes {
                        let (bnx, bny) = dir.next(bx.0, bx.1);
                        map[bny][bnx] = bx.2;
                    }
                    map[y][x] = '.';
                    map[ny][nx] = '@';
                    robotpos = (nx, ny);
                }
            }
            _ => unreachable!(),
        }
    }
    box_gps(&map)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 10092);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 9021);
    }
}
