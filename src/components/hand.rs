use std::{cell::OnceCell, cmp::Ordering, fmt::Display};

use crate::components::{
    card::{Card, CardValue, SUIT},
    support_functions::*,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandValue {
    RoyalFlush = 6,
    StraightFlush = 5,
    ThreeOfAKind = 4,
    Straight = 3,
    Flush = 2,
    Pair = 1,
    HighCard = 0,
}

impl Display for HandValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use HandValue::*;
        f.write_str(match self {
            RoyalFlush => "Royal Flush",
            StraightFlush => "Straight Flush",
            ThreeOfAKind => "Three of a Kind",
            Straight => "Straight",
            Flush => "Flush",
            Pair => "Pair",
            HighCard => "High Card",
        })
    }
}

#[derive(Clone, Debug)]
pub struct Hand {
    pub holder: [Card; 3],
    pub has_ace: bool,
    pub value: OnceCell<HandValue>,
}

impl Hand {
    pub fn new(cards: [Card; 3]) -> Self {
        let mut c = cards;
        c.sort();
        let has_ace = c[2].value == 14;
        Self {
            holder: c,
            has_ace: has_ace,
            value: OnceCell::new(),
        }
    }

    pub fn get_hand_value(&self) -> HandValue {
        *self.value.get_or_init(|| match self {
            x if is_royal_flush(x) => HandValue::RoyalFlush,
            x if is_straight_flush(x) => HandValue::StraightFlush,
            x if is_three_of_a_kind(x).is_some() => HandValue::ThreeOfAKind,
            x if is_straight(x) => HandValue::Straight,
            x if is_flush(x) => HandValue::Flush,
            x if get_a_pair(x).is_some() => HandValue::Pair,
            _ => HandValue::HighCard,
        })
    }

    pub fn key(&self) -> (HandValue, u8, u8, u8) {
        let hv = self.get_hand_value();

        use HandValue::*;
        match hv {
            RoyalFlush => (hv, 0, 0, 0),
            ThreeOfAKind => (hv, self.holder[0].value, 0, 0),
            Pair => {
                let pair = get_a_pair(self).expect("failed to get a pair somehow");
                let v2 = get_highest_card_in_pair(self);
                (hv, pair, v2, 0)
            }
            _ => {
                let [a, b, c] = &self.holder;
                (hv, c.value, b.value, a.value)
            }
        }
    }
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            holder: [
                Card::new(CardValue::TWO, SUIT::HEARTS),
                Card::new(CardValue::FOUR, SUIT::SPADES),
                Card::new(CardValue::ACE, SUIT::SPADES),
            ],
            has_ace: true,
            value: OnceCell::new(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.holder == other.holder && self.get_hand_value() == other.get_hand_value()
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}: {}",
            self.holder[0],
            self.holder[1],
            self.holder[2],
            self.get_hand_value()
        ))
    }
}
