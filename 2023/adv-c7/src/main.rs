use itertools::*;

fn main() {
    let input = include_str!("../input.txt");
    let mut card_hands = vec![];
    for line in input.lines() {
        let splits: Vec<&str> = line.split_ascii_whitespace().collect();
        let cards: Vec<char> = splits[0].chars().collect();
        let bid = splits[1].parse().unwrap();
        card_hands.push(CardHand { cards, bid });
    }

    card_hands.sort_unstable();
    for hand in &card_hands {
        let handtype = hand.hand_type();
        println!("hand is {handtype:?}: {hand:?}  ")
    }
    let mut tot_points = 0;
    for (i, hand) in card_hands.iter().enumerate() {
        let points = (i + 1) * hand.bid;
        tot_points += points;
    }
    println!("{tot_points}");
}
#[derive(Debug)]
struct CardHand {
    cards: Vec<char>,
    bid: usize,
}
impl CardHand {
    fn is_five_kind(&self) -> bool {
        let jokers = self.jokers();
        let card_sorted: Vec<usize> = self.cards_counted();
        match card_sorted.get(0) {
            Some(cards) => cards + jokers == 5,
            None => jokers == 5,
        }
    }
    fn is_four_kind(&self) -> bool {
        let jokers = self.jokers();
        let card_sorted: Vec<usize> = self.cards_counted();
        match card_sorted.get(0) {
            Some(cards) => cards + jokers == 4,
            None => false,
        }
    }
    fn is_tree_kind(&self) -> bool {
        let jokers = self.jokers();
        let card_sorted: Vec<usize> = self.cards_counted();
        card_sorted[0] + jokers == 3
    }
    fn is_house(&self) -> bool {
        let jokers = self.jokers();
        let card_sorted: Vec<usize> = self.cards_counted();
        card_sorted.len() >= 2 && card_sorted[0] + jokers == 3 && card_sorted[1] == 2
            || card_sorted.len() >= 2 && card_sorted[0] == 3 && card_sorted[1] + jokers == 2
    }

    fn is_two_pair(&self) -> bool {
        let jokers = self.jokers();
        let card_sorted: Vec<usize> = self.cards_counted();
        card_sorted.len() >= 2 && card_sorted[0] + jokers == 2 && card_sorted[1] == 2
            || card_sorted.len() >= 2 && card_sorted[0] == 2 && card_sorted[1] + jokers == 2
    }
    fn is_pair(&self) -> bool {
        self.num_diffrent() == 4
    }
    fn num_diffrent(&self) -> usize {
        self.cards_counted().len()
    }
    fn hand_type(&self) -> HandTypes {
        if self.is_five_kind() {
            HandTypes::FiveKind
        } else if self.is_four_kind() {
            HandTypes::FourKind
        } else if self.is_house() {
            HandTypes::FullHouse
        } else if self.is_tree_kind() {
            HandTypes::TreeKind
        } else if self.is_two_pair() {
            HandTypes::TwoPair
        } else if self.is_pair() {
            HandTypes::Pair
        } else {
            HandTypes::HighCard
        }
    }
    fn jokers(&self) -> usize {
        self.cards.iter().filter(|c| **c == 'J').count()
    }
    fn cards_counted(&self) -> Vec<usize> {
        self.cards
            .iter()
            .filter(|c| **c != 'J')
            .counts()
            .iter()
            .map(|(_c, u)| *u)
            .sorted()
            .rev()
            .collect()
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type() != other.hand_type() {
            self.hand_type().cmp(&other.hand_type())
        } else {
            let mut unique_card_idx = 0;
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    unique_card_idx = i;
                    break;
                }
            }
            self.cards[unique_card_idx].cmp(&other.cards[unique_card_idx])
        }
    }
}
impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type() != other.hand_type() {
            self.hand_type().partial_cmp(&other.hand_type())
        } else {
            let mut unique_card_idx = 0;
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    unique_card_idx = i;
                    break;
                }
            }
            let card2: CardTypes = self.cards[unique_card_idx].into();
            let card1: CardTypes = other.cards[unique_card_idx].into();
            card1.partial_cmp(&card2)
        }
    }
}
impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() != other.hand_type()
    }
}
impl Eq for CardHand {}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandTypes {
    HighCard,
    Pair,
    TwoPair,
    TreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CardTypes {
    A,
    K,
    Q,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    J,
    Unknown,
}
impl Into<CardTypes> for char {
    fn into(self) -> CardTypes {
        match self {
            'A' => CardTypes::A,
            'K' => CardTypes::K,
            'Q' => CardTypes::Q,
            'J' => CardTypes::J,
            'T' => CardTypes::T,
            '9' => CardTypes::C9,
            '8' => CardTypes::C8,
            '7' => CardTypes::C7,
            '6' => CardTypes::C6,
            '5' => CardTypes::C5,
            '4' => CardTypes::C4,
            '3' => CardTypes::C3,
            '2' => CardTypes::C2,
            _ => CardTypes::Unknown,
        }
    }
}
const TESTINPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::CardHand;

    #[test]
    fn pair() {
        let cardhand = CardHand {
            cards: vec!['J', '2', '3', '4', '6'],
            bid: 20,
        };
        assert_eq!(cardhand.num_diffrent(), 4);
    }
    #[test]
    fn tree() {
        let cardhand = CardHand {
            cards: vec!['J', '2', '6', '4', '4'],

            bid: 20,
        };
        assert!(cardhand.is_tree_kind());
    }
    #[test]
    fn four() {
        let cardhand = CardHand {
            cards: vec!['J', '8', 'J', '8', 'T'],
            bid: 20,
        };
        assert!(cardhand.is_four_kind());
    }
    #[test]
    fn house() {
        let cardhand = CardHand {
            cards: vec!['J', '3', '3', '3', '7'],
            bid: 20,
        };
        assert!(cardhand.is_house());
    }
}
