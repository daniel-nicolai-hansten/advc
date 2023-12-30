use aoc_runner_derive::{aoc, aoc_generator};
use itertools::*;
#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<CardHand> {
    let mut card_hands = vec![];
    for line in input.lines() {
        let splits: Vec<&str> = line.split_ascii_whitespace().collect();
        let cards = splits[0].chars().map(|c| c.into()).collect();
        let bid = splits[1].parse().unwrap();
        card_hands.push(CardHand { cards, bid });
    }
    card_hands.sort_unstable();
    card_hands
}

#[aoc(day7, part1)]
fn part1(card_hands: &[CardHand]) -> usize {
    let mut tot_points = 0;
    for (i, hand) in card_hands.iter().enumerate() {
        let points = (i + 1) * hand.bid;
        tot_points += points;
    }
    tot_points
}

#[aoc(day7, part2)]
fn part2(card_hands: &[CardHand]) -> usize {
    let mut tot_points = 0;
    let mut joker_hands = vec![];
    for hand in card_hands.iter() {
        let joker_hand = CardHand {
            cards: hand
                .cards
                .iter()
                .map(|c| match c {
                    CardTypes::J => CardTypes::Joker,
                    c => *c,
                })
                .collect(),
            bid: hand.bid,
        };
        joker_hands.push(joker_hand);
    }
    joker_hands.sort_unstable();
    for (i, hand) in joker_hands.iter().enumerate() {
        let points = (i + 1) * hand.bid;
        tot_points += points;
    }
    tot_points
}

#[derive(Debug)]
struct CardHand {
    cards: Vec<CardTypes>,
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
        self.cards
            .iter()
            .filter(|c| **c == CardTypes::Joker)
            .count()
    }
    fn cards_counted(&self) -> Vec<usize> {
        self.cards
            .iter()
            .filter(|c| **c != CardTypes::Joker)
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
            let card2 = self.cards[unique_card_idx];
            let card1 = other.cards[unique_card_idx];
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
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
    Joker,
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
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    const TESTINPUT: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTINPUT)), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTINPUT)), 5);
    }
}
