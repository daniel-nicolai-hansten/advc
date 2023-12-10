use std::fs;
use std::time::Instant;

fn main() {

    // let input = TESTINPUT;
 
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();

    let mut cards = vec![];
    for line in input.lines() {
        let card: Vec<&str> = line.split(": ").collect();
        let cardnums: Vec<&str> = card[1].split(" | ").collect();
        let winning_num: Vec<i32> = cardnums[0]
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let have_num: Vec<i32> = cardnums[1]
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        cards.push(Card {
            winning_num,
            have_num,
            antall: 1,
        })
    }
    for i in 0..cards.len() {
        let card = &cards[i];
        let mut points = 0;
        for num in &card.have_num {
            if card.winning_num.iter().find(|x| x == &num).is_some() {
                points += 1;
            }
        }
        for j in 1..=points {
            let idx = i + j;
            if idx < cards.len() {
                cards[idx].antall += cards[i].antall;
            }
        }
    }
    let mut tot = 0;
    for card in cards {
        tot += card.antall;
    }
    let elapsed = now.elapsed();
    println!("{tot}");
    println!("Elapsed: {:.2?}", elapsed);
    // totpoints += points;
}
struct Card {
    winning_num: Vec<i32>,
    have_num: Vec<i32>,
    antall: u32,
}
const TESTINPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
