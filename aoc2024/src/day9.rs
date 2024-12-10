use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dsk {
    File(u32),
    Free,
}
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Dsk> {
    let mut ret = vec![];
    for (n, c) in input.chars().enumerate() {
        for _ in 0..c.to_digit(10).unwrap() {
            let fs = match n % 2 {
                0 => Dsk::File(n as u32 / 2),
                1 => Dsk::Free,
                _ => panic!("Invalid input"),
            };
            ret.push(fs);
        }
    }
    ret
}

#[aoc(day9, part1)]
fn part1(input: &[Dsk]) -> usize {
    let mut dsk = input.to_vec();
    let (mut head, mut tail) = (0, dsk.len() - 1);
    'outer: loop {
        while dsk[head] != Dsk::Free {
            head += 1;
            if head >= tail {
                break 'outer;
            }
        }
        while dsk[tail] == Dsk::Free {
            tail -= 1;
            if head >= tail {
                break 'outer;
            }
        }
        dsk.swap(head, tail);
    }
    dsk.iter().enumerate().fold(0, |acc, (idx, f)| match f {
        Dsk::File(n) => (idx * *n as usize) + acc,
        Dsk::Free => acc,
    })
}

#[aoc(day9, part2)]
fn part2(input: &[Dsk]) -> usize {
    let mut dsk = input.to_vec();
    let (mut head, mut tail) = (0, dsk.len() - 1);
    let mut moved = HashSet::default();
    'outer: loop {
        while dsk[head] != Dsk::Free {
            head += 1;
            if head >= tail {
                break 'outer;
            }
        }
        while dsk[tail] == Dsk::Free {
            tail -= 1;
            if head >= tail {
                break 'outer;
            }
        }

        let size = find_len(&dsk[..=tail]);
        if let Dsk::File(id) = &dsk[tail] {
            if !moved.insert(*id) {
                tail -= size;
                continue 'outer;
            }
        }
        if let Some(freeblock) = find_free_block_of_size(size, &dsk[head..=tail]) {
            for i in 0..size {
                dsk.swap(head + freeblock + i, tail - i);
            }
        }
        tail -= size;
    }
    dsk.iter().enumerate().fold(0, |acc, (idx, f)| match f {
        Dsk::File(n) => (idx * *n as usize) + acc,
        Dsk::Free => acc,
    })
}
fn find_len(inp: &[Dsk]) -> usize {
    match inp {
        [] => 0,
        [Dsk::File(_)] => 1,
        [rest @ .., Dsk::File(n)] => {
            let mut ret = 0;
            for i in (0..rest.len()).rev() {
                match rest[i] {
                    Dsk::File(m) if m == *n => (),
                    _ => {
                        ret = rest.len() - i;
                        break;
                    }
                }
            }
            ret
        }
        _ => 0,
    }
}
fn find_free_block_of_size(size: usize, input: &[Dsk]) -> Option<usize> {
    let mut freelen = 0;
    let mut ret = None;
    for (idx, block) in input.iter().enumerate() {
        match block {
            &Dsk::File(_) => freelen = 0,
            &Dsk::Free => freelen += 1,
        }
        if freelen >= size {
            ret = Some(1 + idx - freelen);
            break;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "2333133121414131402";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        let test = parse(TESTINPUT);
        assert_eq!(find_len(&test), 2);
        assert_eq!(test[find_free_block_of_size(3, &test[3..]).unwrap() + 3], Dsk::Free);
        assert_eq!(part2(&parse(TESTINPUT)), 2858);
    }
}
