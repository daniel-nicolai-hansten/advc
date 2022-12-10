#[allow(unused_variables)]
use std::fs;
const NUM: usize = 1000;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut grid = [[false; NUM]; NUM];
    let mut rope = [RopePos {
        x: NUM / 2,
        y: NUM / 2,
    }; 10];
    let mut visited = 0;
    for line in input.lines() {
        let mut dir = Direction::Noop;
        match line.chars().nth(0).unwrap() {
            'U' => dir = Direction::Up,
            'D' => dir = Direction::Down,
            'R' => dir = Direction::Right,
            'L' => dir = Direction::Left,
            _ => {}
        }
        let moves = line[2..].parse::<usize>().unwrap();
        for _ in 0..moves {
            rope[0].move_pos(&dir);
            for i in 0..9 {
                rope[i + 1].move_tail(rope[i]);
                if !grid[rope[9].y][rope[9].x] {
                    grid[rope[9].y][rope[9].x] = true;
                    visited += 1;
                }
            }
        }
    }
    println!("visited {}", visited);
}

const TESTINPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Noop,
}

#[derive(Copy, Clone)]
struct RopePos {
    x: usize,
    y: usize,
}
impl RopePos {
    fn move_pos(&mut self, dir: &Direction) {
        match &dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            _ => {}
        }
    }
    fn move_tail(&mut self, head: RopePos) {
<<<<<<< HEAD

=======
        if head.x > self.x + 1 {
            if head.y > self.y {
                self.y += 1;
            }
            if head.y < self.y {
                self.y -= 1;
            }
            self.x += 1;
        }
        if head.y > self.y + 1 {
            if head.x > self.x {
                self.x += 1;
            }
            if head.x < self.x {
                self.x -= 1;
            }
            self.y += 1;
        }
>>>>>>> e896508beadae9f354679299e0cde793f3a5232b
        if head.x < self.x - 1 {
            if head.y > self.y {
                self.y += 1;
            }
            if head.y < self.y {
                self.y -= 1;
            }
            self.x -= 1;
        }
        if head.y < self.y - 1 {
            if head.x > self.x {
                self.x += 1;
            }
            if head.x < self.x {
                self.x -= 1;
            }
            self.y -= 1;
        }
        if head.x > self.x + 1 {
            if head.y != self.y {
                self.y = head.y;
            }
            self.x += 1;
        }
        if head.y > self.y + 1 {
            if head.x != self.x {
                self.x = head.x;
            }
            self.y += 1;
        }
    }
}
fn grid_printer(grid: [[bool; NUM]; NUM]) {
    for row in grid {
        for seen in row {
            if seen {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
