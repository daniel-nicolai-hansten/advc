use std::fs;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        sum += from_snafu(line)
    }
    println!("Sum {}", to_snafu(sum));


}
fn from_snafu(input: &str) -> i128 {
    let mut ret: i128 = 0;
    let len = input.len();
    for (i, c) in input.chars().enumerate() {
        let num = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => 0,
        };
        let num_pos = len - (i + 1);
        if num_pos == 0 {
            ret += num;
        } else {
            ret += num * (5_i128.pow(num_pos as u32));
        }
    }
    ret
}
fn sum_snafu_arr(snafu_arr: &Vec<i128>) -> i128 {
    let len = snafu_arr.len();
    let mut ret = 0;
    for i in 0..len {
        let num = snafu_arr[i];
        let num_pos = len - (i + 1);
        if num_pos == 0 {
            ret += num;
        } else {
            ret += num * (5_i128.pow(num_pos as u32));
        }
    }
    ret
}
fn to_snafu(num: i128) -> String {
    let mut workingnum = num;
    let mut digits = 0;
    while workingnum > 1 * 5_i128.pow(digits) {
        digits += 1;
    }
    let mut snafunum = vec![2; digits as usize];
    println!("digits {}, {}", digits, 1 * 5_i128.pow(digits));
    for power in 0..digits {
        while sum_snafu_arr(&snafunum) >= workingnum {
            snafunum[power as usize] -= 1;
        }
        snafunum[power as usize] += 1;
    }
println!("{:?}", snafunum);
    let mut ret = "".to_owned();
    for i in 0..digits {
       let digit = match snafunum[i as usize] {
            2 => "2",
            1 => "1",
            0 => "0",
            -1 => "-",
            -2 => "=",
            _ => "",
        };
        ret.push_str(digit);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
    #[test]
    fn from_snafu_individual() {
        assert_eq!(from_snafu("2"), 2);
        assert_eq!(from_snafu("1="), 3);
        assert_eq!(from_snafu("1-"), 4);
        assert_eq!(from_snafu("10"), 5);
        assert_eq!(from_snafu("100"), 25);
        assert_eq!(from_snafu("2-"), 9);
        assert_eq!(from_snafu("20"), 10);
        assert_eq!(from_snafu("1=0"), 15);
        assert_eq!(from_snafu("1-0"), 20);
        assert_eq!(from_snafu("120"), 35);
        assert_eq!(from_snafu("1121-1110-1=0"), 314159265);
    }
    #[test]
    fn from_snafu_total() {
        let mut num = 0;
        for line in TESTINPUT.lines() {
            num += from_snafu(line);
        }
        assert_eq!(num, 4890);
    }
    #[test]
    fn to_snafu_num() {
        assert_eq!(to_snafu(35), "120");
    }
}
