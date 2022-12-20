use std::{ops::Index, fs};

const LEN: usize = 7;
fn main() {
    
    let mut cordinatefile2 = vec![];
    let mut idx = 0;
    let input = fs::read_to_string("./input.txt").unwrap();
    for line in TESTINPUT.lines() {
        let num = isize::from_str_radix(line, 10).unwrap();
        
        cordinatefile2.push((num, 0));
        idx += 1;
    }
    let mut i = 0;
    while i < LEN {
        //println!("{:?}", cordinatefile2);
        let (currentnum, sorted) = cordinatefile2[i];
        if sorted == 0 {
            let newpos = calculate_offset(i, currentnum);
            println!("newpos: {}  num: {}  index{}", newpos, currentnum, i);

            cordinatefile2.remove(i);
            cordinatefile2.insert(newpos, (currentnum, sorted + 1));
        } else {
            i += 1;
        }
    }
    println!("Final: {:?}", cordinatefile2);
    let mut nullpos = 0;
    for (pos, (num, _)) in cordinatefile2.iter().enumerate() {
        if *num == 0 {
            println!("{}", pos);
            nullpos = pos;
        }
    }
    let index1 = (nullpos + 1000) % LEN;
    let index2 = (nullpos + 2000) % LEN;
    let index3 = (nullpos + 3000) % LEN;
    let (num1, _) = cordinatefile2[index1];
    let (num2, _) = cordinatefile2[index2];
    let (num3, _) = cordinatefile2[index3];
    println!(
        "1:{} 2:{} 3:{} sum: {}",
        num1,
        num2,
        num3,
        num1 + num2 + num3
    );
}
fn calculate_offset(index: usize, number: isize) -> usize {
    let mut ret = (index as isize + number).rem_euclid(LEN as isize);
    ret as usize 
}
const TESTINPUT: &str = "1
2
-3
3
-2
0
4
";

#[cfg(test)]
mod tests {
    use crate::calculate_offset;

    #[test]
    fn it_works() {
        assert_eq!(calculate_offset(2, 2), 4);
        assert_eq!(calculate_offset(2, 8), 3);
    }
    #[test]
    fn it_works_with_negative() {
        assert_eq!(calculate_offset(2, -2), 0);
        assert_eq!(calculate_offset(2, -8), 1);
        assert_eq!(calculate_offset(2, -3), 6);
    }
}
