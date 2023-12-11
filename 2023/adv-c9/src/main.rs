fn main() {
    let input = include_str!("../input.txt");
    let oasis_historys: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let p1 = oasis_historys
        .iter()
        .fold(0, |acc, v| acc + predict(v, Dir::Fwd));
    let p2 = oasis_historys
        .iter()
        .fold(0, |acc, v| acc + predict(v, Dir::Rev));
    println!("Pt1: {p1} Pt2: {p2}");
}

fn predict(values: &[i32], dir: Dir) -> i32 {
    let diffs: Vec<i32> = values.windows(2).map(|v| v[1] - v[0]).collect();
    let allzeros = !values.iter().fold(false, |acc, x| acc || *x != 0);
    match (allzeros, dir) {
        (false, Dir::Fwd) => values.last().unwrap() + predict(&diffs, dir),
        (false, Dir::Rev) => values.first().unwrap() - predict(&diffs, dir),
        (true, _) => 0,
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Fwd,
    Rev,
}
