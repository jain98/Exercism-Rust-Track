use std::collections::{HashMap, HashSet};
use std::cmp::{Reverse, Ordering};
use crate::HandType::{HighCard, FourOfAKind, FullHouse, ThreeOfAKind, TwoPair, OnePair, StraightFlush, Straight, Flush};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut result = vec![];
    for s in hands {
        result.push(PokerHand::new(s));
    }
    result.sort_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Less));
    Some(result
        .iter()
        .take_while(|&h| h == result.get(0).unwrap())
        .map(|h| h.orig_hand)
        .collect())
}


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug)]
struct PokerHand<'a> {
    orig_hand: &'a str,
    cards: Vec<(u8, u8)>,
    hand_type: HandType
}

impl<'a> PokerHand<'a> {
    fn new(s: &'a str) -> Self {
        PokerHand::from(s)
    }
}

// Create a hand from str
impl<'a> From<&'a str> for PokerHand<'a> {
    fn from(s: &'a str) -> Self {
        let mut counter: HashMap<u8, u8> = HashMap::new();
        let mut suits: HashSet<&str> = HashSet::new();

        s.split_whitespace().for_each(|c| {
            let (v, suit) = c.split_at(c.len() - 1);
            let val: u8 = match v {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                _ => v.parse().unwrap_or_else(|_| panic!("\n\nGot invalid card: {}", v) )
            };

            *counter.entry(val).or_default() += 1;
            suits.insert(suit);
        });

        let mut cards: Vec<(u8, u8)> = counter.drain().collect();
        cards.sort_by_key(|&(val, cnt)| (Reverse(cnt), Reverse(val)) );

        let counts = cards.iter().map(|c| c.1).collect::<Vec<u8>>();
        let mut hand_type = match counts[..] {
            [1,1,1,1,1] => {  // May be Straight
                let faces = cards.iter().map(|c| c.0).collect::<Vec<u8>>();
                let mut hand_type: HandType = Straight;

                // five-high straight flush is the lowest combo in straights, so modify cards vector to reflect so
                if faces[..] == [14, 5, 4, 3, 2] {
                    cards.remove(0);
                    cards.push((1, 1));
                } else {
                    let required_diff = 1;
                    for f in faces.windows(2) {
                        if f[0] - f[1] != required_diff {
                            hand_type = HighCard;
                            break;
                        }
                    }
                }
                hand_type
            },
            [4, 1] => FourOfAKind,
            [3, 2] => FullHouse,
            [3, 1, 1] => ThreeOfAKind,
            [2, 2, 1] => TwoPair,
            [2, 1, 1, 1] => OnePair,
            _ => HighCard
        };

        hand_type = if suits.len() == 1 {
            if hand_type == Straight {
                StraightFlush
            } else {
                Flush
            }
        } else {
            hand_type
        };

        PokerHand {
            orig_hand: s,
            cards,
            hand_type
        }
    }
}

impl PartialOrd for PokerHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                Some(other.cards.cmp(&self.cards))
            },
            _ => Some(self.hand_type.cmp(&other.hand_type))
        }
    }
}

impl PartialEq for PokerHand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}
