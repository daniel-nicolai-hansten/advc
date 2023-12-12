use std::{cmp::max, fs};

fn main() {
    // let input = TESTINPUT;
    let input = fs::read_to_string("input.txt").unwrap();
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
    // fn handtype(&self) -> HandTypes {
    //     if self.cards
    // }
    fn is_five_kind(&self) -> bool {
        self.num_diffrent() == 1
    }
    fn is_four_kind(&self) -> bool {
        let mut num_typ1 = 0;
        let mut num_typ2 = 0;
        let card1 = self.cards[0];
        let mut card2 = None;
        for card in &self.cards {
            if card != &card1 {
                card2 = Some(card);
            }
        }
        for card in &self.cards {
            if card == &card1 {
                num_typ1 += 1;
            } else if card == card2.unwrap() {
                num_typ2 += 1;
            }
        }
        max(num_typ1, num_typ2) == 4
    }
    fn is_tree_kind(&self) -> bool {
        let mut num_typ1 = 0;
        let mut num_typ2 = 0;
        let mut num_typ3 = 0;
        let card1 = self.cards[0];
        let mut card2 = None;
        let mut card3 = None;
        for card in &self.cards {
            if card2.is_none() && card != &card1 {
                card2 = Some(card);
            } else if card3.is_none() && card != &card1 && card != card2.unwrap() {
                card3 = Some(card);
            }
        }
        for card in &self.cards {
            if card == &card1 {
                num_typ1 += 1;
            } else if card == card2.unwrap() {
                num_typ2 += 1;
            } else if card == card3.unwrap() {
                num_typ3 += 1;
            }
        }
        let mut hand_sorted = vec![num_typ1, num_typ2, num_typ3];
        hand_sorted.sort_unstable();
        hand_sorted[0] == 1 && hand_sorted[1] == 1 && hand_sorted[2] == 3
    }
    fn is_house(&self) -> bool {
        let mut num_typ1 = 0;
        let mut num_typ2 = 0;
        let card1 = self.cards[0];
        let mut card2 = None;
        for card in &self.cards {
            if card2.is_none() && card != &card1 {
                card2 = Some(card);
            }
        }
        for card in &self.cards {
            if card == &card1 {
                num_typ1 += 1;
            } else if card == card2.unwrap() {
                num_typ2 += 1;
            }
        }
        let mut hand_sorted = vec![num_typ1, num_typ2];
        hand_sorted.sort_unstable();
        hand_sorted[0] == 2 && hand_sorted[1] == 3
    }
    fn is_two_pair(&self) -> bool {
        let mut num_typ1 = 0;
        let mut num_typ2 = 0;
        let mut num_typ3 = 0;
        let card1 = self.cards[0];
        let mut card2 = None;
        let mut card3 = None;
        for card in &self.cards {
            if card2.is_none() && card != &card1 {
                card2 = Some(card);
            } else if card3.is_none() && card != &card1 && card != card2.unwrap() {
                card3 = Some(card);
            }
        }
        for card in &self.cards {
            if card == &card1 {
                num_typ1 += 1;
            } else if card == card2.unwrap() {
                num_typ2 += 1;
            } else if card == card3.unwrap() {
                num_typ3 += 1;
            }
        }
        let mut hand_sorted = vec![num_typ1, num_typ2, num_typ3];
        hand_sorted.sort_unstable();
        hand_sorted[0] == 1 && hand_sorted[1] == 2 && hand_sorted[2] == 2
    }
    fn is_pair(&self) -> bool {
        self.num_diffrent() == 4
    }
    fn num_diffrent(&self) -> usize {
        let mut ord_cards = self.cards.clone();
        ord_cards.sort_unstable();
        let mut num_diffrent_card = 0;
        let mut lastcard = None;
        for card in ord_cards {
            if !(Some(card) == lastcard) {
                num_diffrent_card += 1;
                lastcard = Some(card);
            }
        }
        num_diffrent_card
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
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
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
