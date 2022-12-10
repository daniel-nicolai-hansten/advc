#[allow(unused_variables)]
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut register = 1;
    let mut clock = 0;
    let mut que = Que{que: [0; 1024], front: 0, back: 0,};
    let mut result = vec![];
    for line in input.lines() {
        match &line[..4] {
            "noop" => {
                que.add(0);
            },
            "addx" => {
                que.add(0);
                que.add(line[5..].parse::<i32>().unwrap());
                //print!("{}", register);
            },
            _ => println!("{}", line),
        }
    }
    loop {
        clock += 1;
        let pos = (clock % 40) -1;
        if !(register >= (pos - 1) && register <= (pos +1)) {
            print!(" ");
            //print!(". P{} r{}", pos, register);
        } else {
            print!("#");
            //print!("#P{} r{}", pos, register);
        }
        match clock {
            40 | 80 | 120 | 160 | 200 | 240  => {
                //println!("{} {} {}", clock * register, clock, register);
                println!("");
                result.push(clock * register);
            },
            _ => {},
            }
    if let Some(val) = que.get(){
        register += val;
        //if val != 0 && clock > 180 {println!("{}", val);}
    }


    if clock >= 240 {break;}
    }
    println!("Sum {}", result.iter().sum::<i32>());
}
struct Que  {
    que: [i32; 1024],
    front: usize,
    back: usize,
}
impl Que {
    fn add(&mut self, indata: i32){
        self.que[self.back] = indata;
        self.back = (self.back +1) & 0x3ff;
        //print!("in:{} ", indata);    
    }
    fn get(&mut self) -> Option<i32>{
        let mut ret = None;
        if self.front != self.back {
            ret = Some(self.que[self.front]);
            self.front = (self.front +1) & 0x3ff;
            //print!("out:{} ", ret.unwrap());
        }
        ret
    }
}
const TESTINPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
