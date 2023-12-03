use std::fs;
const NUM: u32 = 99;
const NUM2: usize = 99;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut tree_grid: [[(u32, bool); NUM2]; NUM2] = [[(0, false); NUM2]; NUM2];
    let mut trees_viewing_distance: Vec<usize> = vec![];
    let mut x: i32;
    let mut y: i32 = 0;
    for line in input.lines() {
        x = 0;
        for c in line.chars() {
            tree_grid[y.try_into().unwrap_or(0)][x.try_into().unwrap_or(0)] =
                (c.to_digit(10).unwrap() + 1, false);
            x += 1;
        }
        y += 1;
    }
    let dirs = [LEFT, TOP, RIGHT, BOTTOM];
    let mut trees_seen = 0;
    for dir in dirs {
        x = dir.start_x.try_into().unwrap();

        y = dir.start_y.try_into().unwrap();

        for _ in 0..NUM {
            let mut tallest = 0;
            if dir.x_inc2 != 0 {
                x = dir.start_x.try_into().unwrap();
            }
            if dir.y_inc2 != 0 {
                y = dir.start_y.try_into().unwrap();
            }
            for _ in 0..NUM {
                let (tree, counted) =
                    tree_grid[y.try_into().unwrap_or(0)][x.try_into().unwrap_or(0)];
                //println!("x: {x} y: {y}");
                if tree > tallest {
                    tallest = tree;
                    if !counted {
                        trees_seen += 1;
                        tree_grid[y.try_into().unwrap_or(0)][x.try_into().unwrap_or(0)] =
                            (tree, true);
                    }
                }
                x = x + dir.x_inc2;
                y = y as i32 + dir.y_inc2;
            }
            x = x as i32 + dir.x_inc1;
            y = y as i32 + dir.y_inc1;
        }
    }
    tree_grid_printer(tree_grid);
    println!("Seen {}", trees_seen);
    let mut curretpos = Pos { x: 0, y: 0 };
    for row in tree_grid {
        curretpos.x = 0;
        for (tree, _) in row {
            let mut view_dist = [0, 0, 0, 0];
            let dirs2 = [LEFT, TOP, RIGHT, BOTTOM];
            let mut i = 0;
            for dir in dirs2 {
                (x, y) = (curretpos.x, curretpos.y);
                for _ in 0..NUM {
                    //                    println!("currentpos: {} {} x: {} y: {}", curretpos.x, curretpos.y, x ,y);

                    x = x + dir.x_inc2;
                    y = y + dir.y_inc2;
                    if x >= NUM.try_into().unwrap()
                        || y >= NUM.try_into().unwrap()
                        || x < 0
                        || y < 0
                    {
                        break;
                    }
                    view_dist[i] += 1;
                    let (nbtree, _) =
                        tree_grid[y.try_into().unwrap_or(0)][x.try_into().unwrap_or(0)];
                    if tree <= nbtree {
                        break;
                    }
                }
                i += 1;
            }
            //            println!(
            //                "{} {} {} {}",
            //                view_dist[0], view_dist[1], view_dist[2], view_dist[3]
            //            );
            let view_distance = view_dist[0] * view_dist[1] * view_dist[2] * view_dist[3];
            trees_viewing_distance.push(view_distance);
            curretpos.x += 1;
        }
        curretpos.y += 1;
    }
    trees_viewing_distance.sort();
    println!("{:?}", trees_viewing_distance.last().unwrap());
}

fn tree_grid_printer(grid: [[(u32, bool); NUM2]; NUM2]) {
    for row in grid {
        for (tree, seen) in row {
            if seen {
                //print!("{}",'\u{1F332}')
                print!("\x1b[92m{}\x1b[0m", "^");
            } else {
                print!("{}", ".");
            }
        }
        println!("");
    }
}
// #[derive(Debug, EnumIter)]
// enum Dir {
//     Left,
//     Right,
//     Top,
//     Bottom,

// }
struct Pos {
    x: i32,
    y: i32,
}
struct Dir {
    start_x: u32,
    start_y: u32,
    x_inc1: i32,
    x_inc2: i32,
    y_inc1: i32,
    y_inc2: i32,
}
const LEFT: Dir = Dir {
    start_x: 0,
    start_y: 0,
    x_inc1: 0,
    x_inc2: 1,
    y_inc1: 1,
    y_inc2: 0,
};
const RIGHT: Dir = Dir {
    start_x: NUM - 1,
    start_y: 0,
    x_inc1: 0,
    x_inc2: -1,
    y_inc1: 1,
    y_inc2: 0,
};
const TOP: Dir = Dir {
    start_x: 0,
    start_y: 0,
    x_inc1: 1,
    x_inc2: 0,
    y_inc1: 0,
    y_inc2: 1,
};

const BOTTOM: Dir = Dir {
    start_x: 0,
    start_y: NUM - 1,
    x_inc1: 1,
    x_inc2: 0,
    y_inc1: 0,
    y_inc2: -1,
};
// fn count_tree (mut tree_grid: [[(u32, bool); NUM]; NUM]) {

// }
const TESTINPUT: &str = "30373
25512
65332
33549
35390";
