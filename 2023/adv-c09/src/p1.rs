fn main() {
    // let input = TESTINPUT;
    let input = include_str!("../input.txt");
    let mut oasis_historys: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        oasis_historys.push(nums);
    }
    let mut p1 = 0;
    for history in oasis_historys {
        p1 += predict(&history);
    }
    println!("{p1}");
}
fn predict(values: &[i32]) -> i32 {
    let diffs: Vec<i32> = values.windows(2).map(|v| v[1] - v[0]).collect();
    let mut ret = 0;
    if values.iter().fold(false, |acc, x| acc || *x != 0) {
        ret = predict(&diffs) + values.last().unwrap();
    }
    ret
}
const TESTINPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
