use std::time::Instant;
use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};
extern crate aoc_runner;

extern crate aoc_runner_derive;

fn find_hash(instr: &str) -> usize {
    let mut currval = 0;
    for c in instr.chars() {
        currval += c as usize;
        currval *= 17;
        currval %= 256;
    }

    currval
}
#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<String> {
    let input = strip_trailing_newline(input);
    input.split(",").map(|s| s.to_string()).collect()

}
#[aoc(day15, part1)]
fn p1(input:&[String]) -> u32 {
    input.iter().map(|s| find_hash(s)).sum::<usize>() as u32
}

#[aoc(day15, part2)]
fn p2(input:&[String]) -> u32 {
    let _now = Instant::now();
    let mut boxes: Vec<Vec<_>> = vec![];
    for _ in 0..256 {
        boxes.push(vec![]);
    }
    for lens in input {
        let label: Vec<&str> = lens.split(&['-', '=']).collect();
        let boxnum = find_hash(label[0]);
        if let Ok(num) = label[1].parse::<usize>() {
            if let Some(ptr) = boxes[boxnum].iter_mut().find(|(s, _)| *s == label[0]) {
                *ptr = (label[0], num);
            } else {
                boxes[boxnum].push((label[0], num));
            }
        } else {
            if let Some((n, _)) = boxes[boxnum].iter().enumerate().find(|(_n, (s, _))| *s == label[0]) {
                boxes[boxnum].remove(n);
            }
        }
    }
    let mut sum = 0;
    for (n, boxn) in boxes.iter().enumerate() {
        let boxval = n + 1;
        let sumadd: usize = boxn.iter().enumerate().map(|(idx, (_, val))| boxval * (idx +1) * val).sum();
        sum += sumadd;
    }
    sum as u32
}


fn strip_trailing_newline(input: &str) -> &str {
    input.strip_suffix("\r\n").or(input.strip_suffix("\n")).unwrap_or(input)
}


aoc_lib!{ year = 2023 }