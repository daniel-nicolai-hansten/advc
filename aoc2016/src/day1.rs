use aoc_runner_derive::{aoc, aoc_generator};
use common::pos::*;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<(char, usize)> {
    input
        .split(", ")
        .map(|line| {
            let (turn, steps) = line.split_at(1);
            let direction = match turn {
                "L" => 'L',
                "R" => 'R',
                _ => panic!("Invalid turn direction"),
            };
            let steps: usize = steps.trim().parse().expect("Invalid step count");
            (direction, steps)
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(char, usize)]) -> usize {
    let (endpos, _) = input.iter().fold((IPosition::new(), Direction::North), |(mut position, mut direction), (turn, steps)| {
        direction = match turn {
            'L' => direction.turn_left(),
            'R' => direction.turn_right(),
            _ => panic!("Invalid turn direction"),
        };
        position = position.move_in_direction(&direction, *steps).unwrap();
        (position, direction)
    });
    endpos.distance(&IPosition::new())
}

#[aoc(day1, part2)]
fn part2(input: &[(char, usize)]) -> usize {
    let mut visited = std::collections::HashSet::new();
    visited.insert(IPosition::new());
    if let Err((endpos, _)) = input.iter().try_fold((IPosition::new(), Direction::North), |(mut position, mut direction), (turn, steps)| {
        direction = match turn {
            'L' => direction.turn_left(),
            'R' => direction.turn_right(),
            _ => panic!("Invalid turn direction"),
        };
        for _ in 0..*steps {
             position = position.move_in_direction(&direction, 1).unwrap();
             match visited.insert(position) {
                 true => (),
                 false => return Err((position, direction)),
             }
        }
        Ok((position, direction))
    }) {
        return endpos.distance(&IPosition::new());
    } else {
        panic!("No position visited twice");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
