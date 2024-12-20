use std::collections::HashSet;

#[allow(dead_code)]
fn substringfinder(s: &str) -> i32 {
    let lst = s.as_bytes();
    let (mut head, mut tail, mut ans) = (0, 0, 0);
    let mut map: u128 = 0;
    while head < lst.len() {
        match map.count_ones() as usize == (head - tail) {
            true => {
                map ^= 1 << (0x7f & lst[head]);
                ans = std::cmp::max(ans, map.count_ones());
                head += 1;
            }
            false => {
                map ^= 1 << (0x7f & lst[tail]);
                tail += 1;
            }
        }
    }
    ans as i32
}
#[allow(dead_code)]
fn substringfinder_hashmap(s: &str) -> i32 {
    let lst = s.as_bytes();
    let (mut head, mut tail, mut ans) = (0, 0, 0);
    let mut map = HashSet::new();
    let mut last = lst[0];
    while head < lst.len() {
        match map.len() == (head - tail) {
            true => {
                map.insert(lst[head]);
                ans = std::cmp::max(ans, map.len());
                last = lst[head];
                head += 1;
            }
            false => {
                if lst[tail] != last {
                    map.remove(&lst[tail]);
                }
                tail += 1;
            }
        }
    }
    ans as i32
}
#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT1: &str = "abcabcbb";
    const TESTINPUT2: &str = "bbbbb";
    const TESTINPUT3: &str = "abbceb";
    #[test]
    fn part1_example() {
        assert_eq!(substringfinder(TESTINPUT1), 3);
        assert_eq!(substringfinder(TESTINPUT2), 1);
        assert_eq!(substringfinder(TESTINPUT3), 3);
        assert_eq!(substringfinder(""), 0);
        assert_eq!(substringfinder(" "), 1);
        assert_eq!(substringfinder("abc"), 3);
        assert_eq!(substringfinder_hashmap(TESTINPUT1), 3);
        assert_eq!(substringfinder_hashmap(TESTINPUT2), 1);
        assert_eq!(substringfinder_hashmap(TESTINPUT3), 3);
        //assert_eq!(substringfinder_hashmap(""), 0);
        assert_eq!(substringfinder_hashmap(" "), 1);
        assert_eq!(substringfinder_hashmap("abc"), 3);
    }
    #[test]
    fn time_largefile() {
        let file = std::fs::read_to_string("/tmp/largetext.txt").unwrap_or("s".to_string());
        let start = std::time::Instant::now();
        let n = substringfinder(&file);
        let elapsed = start.elapsed();
        println!("Time taken to find substring: {:?}, num: {n}", elapsed);
        let start = std::time::Instant::now();
        let n = substringfinder_hashmap(&file);
        let elapsed = start.elapsed();
        println!("Time taken to find substring hm: {:?}, num: {n}", elapsed);
    }
}

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
