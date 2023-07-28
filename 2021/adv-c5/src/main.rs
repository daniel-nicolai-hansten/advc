fn main() {
    println!("Hello, world!");
}
struct SteamMap {
    map: Vec<Vec<u32>>,
}
impl SteamMap {
    fn draw_line(&mut self, line: &str) {
        for p in line.split(" -> ") {
            let (x, y) = p.split(",")[0];
        }
    }
}
#[cfg(test)]
mod test {
    const TESTINPUT: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    #[test]
    fn test() {
        for line in TESTINPUT.lines() {

        }
    }
}

