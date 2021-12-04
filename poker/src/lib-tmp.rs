use std::cmp::Ordering;
use maplit::hashmap;
use std::collections::{HashMap, HashSet};
use crate::HandType::{StraightFlush, FourOfAKind, FullHouse, Flush, Straight, ThreeOfAKind, TwoPair, OnePair, HighCard};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut h = vec![];
    for s in hands {
        h.push(PokerHand::new(s));
    }
    h.sort_by(|x, y| y.partial_cmp(x).unwrap_or(Ordering::Less));
    Some(h.iter().map(|hand| hand.orig_hand).take(1).collect())
}

////////////////////////
////// Hand Type ///////
////////////////////////
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush
}

#[cfg(test)]
mod hand_type_test {
    use crate::HandType::*;

    #[test]
    fn hand_type_order() {
        let mut a = [StraightFlush, FourOfAKind, FullHouse, Flush, Straight, ThreeOfAKind, TwoPair, OnePair, HighCard];
        a.reverse();
        a.sort();
        assert_eq!(a, [StraightFlush, FourOfAKind, FullHouse, Flush, Straight, ThreeOfAKind, TwoPair, OnePair, HighCard]);
    }
}

////////////////////////
//////// Hand /////////
////////////////////////
#[derive(Debug)]
struct PokerHand<'a> {
    orig_hand: &'a str,
    cards: Vec<Card>,
    hand_type: HandType
}

impl<'a> PokerHand<'a> {
    fn new(s: &'a str) -> Self {
        let mut result = PokerHand::from(s);
        result.set_hand_type();
        result
    }

    fn set_hand_type(&mut self) {
        self.hand_type = if self.is_straight_flush() {
            StraightFlush
        } else if self.is_four_of_a_kind() {
            FourOfAKind
        }
        else if self.is_full_house() {
            FullHouse
        }
        else if self.is_flush() {
            Flush
        }
        else if self.is_straight() {
            Straight
        }
        else if self.is_three_of_a_kind() {
            ThreeOfAKind
        }
        else if self.is_two_pair() {
            TwoPair
        }
        else if self.is_one_pair() {
            OnePair
        }
        else {
            HighCard
        };
    }

    fn is_straight_flush(&self) -> bool {
        let is_flush_by_num = if self.cards.get(0).unwrap().number == 14 {
            let at_4 = self.cards.get(4).unwrap().number;

            if at_4 != 2 && at_4 != 10 {
                return false;
            }

            self.cards.get(1).unwrap().number - self.cards.get(4).unwrap().number == 3
        } else {
            self.cards.get(0).unwrap().number - self.cards.get(4).unwrap().number == 4
        };

        let is_flush_by_type = self.cards.iter().map(|c| c.card_type).collect::<HashSet<char>>().len() == 1;

        let mut map: HashMap<i32, i32> = HashMap::new();

        self.cards.iter().map(|c| c.number).for_each(|n| {
            *map.entry(n).or_default() += 1
        });

        is_flush_by_num && is_flush_by_type && map.values().filter(|&v| *v == 1).count() == 5
    }

    fn is_four_of_a_kind(&self) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        self.cards.iter().map(|c| c.number).for_each(|n| {
            *map.entry(n).or_default() += 1
        });
        map.values().any(|v| *v == 4)
    }

    fn is_full_house(&self) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        self.cards.iter().map(|c| c.number).for_each(|n| {
            *map.entry(n).or_default() += 1
        });
        map.values().any(|v| *v == 3) && map.values().any(|v| *v == 2)
    }

    fn is_flush(&self) -> bool {
        self.cards.iter().map(|c| c.card_type).collect::<HashSet<char>>().len() == 1
    }

    fn is_straight(&self) -> bool {
        let is_flush_by_num = if self.cards.get(0).unwrap().number == 14 {
            let at_4 = self.cards.get(4).unwrap().number;

            if at_4 != 2 && at_4 != 10 {
                return false;
            }

            //find difference of first 4 numbers
            self.cards.get(1).unwrap().number - self.cards.get(4).unwrap().number == 3
        } else {
            self.cards.get(0).unwrap().number - self.cards.get(4).unwrap().number == 4
        };

        let mut map: HashMap<i32, i32> = HashMap::new();

        self.cards.iter().map(|c| c.number).for_each(|n| {
            *map.entry(n).or_default() += 1
        });

        is_flush_by_num && map.values().filter(|&v| *v == 1).count() == 5
    }

    fn is_three_of_a_kind(&self) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        self.cards.iter().map(|c| c.number).for_each(|n| {
            *map.entry(n).or_default() += 1
        });
        map.values().any(|v| *v == 3)
    }

    fn is_two_pair(&self) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        self.cards.iter().map(|c| c.number).for_each(|n| {
            *map.entry(n).or_default() += 1
        });
        map.values().filter(|&v| *v == 2).count() == 2
    }

    fn is_one_pair(&self) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        self.cards.iter().map(|c| c.number).for_each(|n| {
            *map.entry(n).or_default() += 1
        });
        map.values().filter(|&v| *v == 2).count() == 1
    }
}

// Create a hand from str
impl<'a> From<&'a str> for PokerHand<'a> {
    fn from(s: &'a str) -> Self {
        // Each card is represented by 2 characters, there are 5 cards, separated by 4 spacess
        if s.len() > 15 {
            panic!("Invalid hand!");
        }

        let mut cards = vec![];
        for card_str in s.split_ascii_whitespace() {
            cards.push(Card::new(card_str));
        }

        cards.sort_by(|a, b| b.partial_cmp(a).unwrap());

        PokerHand {
            orig_hand: s,
            cards,
            hand_type: HandType::HighCard
        }
    }
}

impl PartialOrd for PokerHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            self.cards.partial_cmp(&other.cards)
        } else {
            Some(self.hand_type.cmp(&other.hand_type))
        }
    }
}

impl PartialEq for PokerHand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

#[cfg(test)]
mod hand_tests {
    use crate::PokerHand;

    #[test]
    fn is_straight_flush() {
        let hand = PokerHand::new("7S 8S 9S 6S 10S");
        assert!(hand.is_straight_flush());
    }

    #[test]
    fn is_four_of_a_kind() {
        let hand = PokerHand::new("3S 3H 2S 3D 3C");
        assert!(hand.is_four_of_a_kind())
    }

    #[test]
    fn is_full_house() {
        let hand = PokerHand::new("4S 5C 4C 5D 4H");
        assert!(hand.is_full_house())
    }

    #[test]
    fn is_flush() {
        let hand = PokerHand::new("2S 4S 5S 6S 7S");
        assert!(hand.is_flush())
    }

    #[test]
    fn is_straight() {
        let hand = PokerHand::new("10D JH QS KD AC");
        assert!(hand.is_straight())
    }

    #[test]
    fn is_three_of_a_kind() {
        let hand = PokerHand::new("4S 5H 4C 8S 4H");
        assert!(hand.is_three_of_a_kind())
    }

    #[test]
    fn is_two_pair() {
        let hand = PokerHand::new("4S 5H 4C 8C 5C");
        assert!(hand.is_two_pair())
    }

    #[test]
    fn is_one_pair() {
        let hand = PokerHand::new("2S 8H 6S 8D JH");
        assert!(hand.is_one_pair())
    }
}

////////////////////////
//////// Card /////////
////////////////////////
#[derive(Debug)]
struct Card {
    number: i32,
    card_type: char
}

impl Card {
    fn new(s: &str) -> Card {
        Card::from(s)
    }
}
impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let ranks = hashmap! {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14
    };

        // Each card is represented by max of 3 characters
        if s.len() > 3 {
            panic!("Invalid card!");
        }

        let card_data = s.chars().collect::<Vec<char>>();
        Card {
            number: if card_data.len() == 2 {
                *ranks.get(card_data.get(0).unwrap()).unwrap()
            } else {
                // It's a 10 and we're representing it as T
                10
            },
            card_type: if card_data.len() == 2 {
                *card_data.get(1).unwrap()
            } else {
                *card_data.get(2).unwrap()
            }

        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.number.cmp(&other.number))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

#[cfg(test)]
mod card_tests {
    use crate::Card;

    #[test]
    pub fn test_sorting_cards() {
        let mut cards = [
            Card::new("4S"),
            Card::new("5C"),
            Card::new("4C"),
            Card::new("5D"),
            Card::new("4H")
        ];

        cards.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!([Card::new("4S"), Card::new("4C"), Card::new("4H"), Card::new("5C"), Card::new("5D")], cards);
    }
}