use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()

}

#[aoc(day3, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    let value_table = value_table_generator();
    let mut items = vec![];
    let mut value_sum = 0;
    for line in input {
        let center = line.len()/2;
        let comp1 = &line[..center];
        let comp2 = &line[center..];
        assert_eq!(comp1.len(), comp2.len());
        for c in comp1 {
            if comp2.contains(c) {
                items.push(c);
                break;
            }
        }
    }
    for item in &items {
        value_sum += value_table.get(&item).unwrap_or(&0);
    }
    value_sum
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let value_table = value_table_generator();
    let mut group_items = vec![];
    let mut badges = vec![];
    let mut value_sum = 0;
    let mut elf_num = 0;
    for line in input {
        if elf_num != 2 {
            group_items.insert(elf_num, line);
            elf_num += 1;
        } else {
            let mut token_candidate = vec![];
            for c in line {
                if group_items.get(0).unwrap().contains(c) {
                    token_candidate.push(c);
                }
            }
            for c in token_candidate {
                if group_items.get(1).unwrap().contains(c) {
                    badges.push(c);
                    break;
                }
            }
            elf_num = 0;
        }
                
    }
    for item in &badges {
        value_sum += value_table.get(&item).unwrap_or(&0);
    }
    value_sum
}
fn value_table_generator() -> HashMap<char, usize> {
    let mut val = 1;
    let mut table = HashMap::new();
    for c in 'a'..='z' {
        table.insert(c, val);
        val +=1;
    }
    for c in 'A'..='Z' {
        table.insert(c, val);
        val +=1;
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    const RUGSACK_CONTENTS: &str = 
    r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(RUGSACK_CONTENTS)), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(RUGSACK_CONTENTS)), 70);
    }
}


