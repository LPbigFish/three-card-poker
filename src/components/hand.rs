use std::cmp::{self, Ordering};

use crate::components::{
    card::Card,
    support_functions::{
        get_a_pair, get_highest_card_in_pair, is_flush, is_royal_flush, is_straight,
        is_straight_flush, is_three_of_a_kind,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HAND_VALUE {
    RoyalFlush = 6,
    StraightFlush = 5,
    ThreeOfAKind = 4,
    Straight = 3,
    Flush = 2,
    Pair = 1,
    HighCard = 0,
}

pub struct Hand {
    pub holder: [Card; 3],
    pub has_ace: bool,
    pub value: Option<HAND_VALUE>,
}

impl Hand {
    pub fn new(cards: [Card; 3]) -> Self {
        let mut c = cards;
        c.sort();
        let has_ace = c[2].value == 14;
        Self {
            holder: c,
            has_ace: has_ace,
            value: None,
        }
    }

    pub fn get_hand_value(&self) -> HAND_VALUE {
        if let Some(s) = &self.value {
            return *s;
        }

        match self {
            x if is_royal_flush(x) => HAND_VALUE::RoyalFlush,
            x if is_straight_flush(x) => HAND_VALUE::StraightFlush,
            x if is_three_of_a_kind(x).is_some() => HAND_VALUE::ThreeOfAKind,
            x if is_straight(x) => HAND_VALUE::Straight,
            x if is_flush(x) => HAND_VALUE::Flush,
            x if get_a_pair(x).is_some() => HAND_VALUE::Pair,
            _ => HAND_VALUE::HighCard,
        }
    }

    pub fn set_hand_value(&mut self) {
        self.value = Some(self.get_hand_value())
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.holder == other.holder && self.get_hand_value() == other.get_hand_value()
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_value_1 = self.get_hand_value();
        let hand_value_2 = other.get_hand_value();

        match hand_value_1 {
            x if x > hand_value_2 => Some(Ordering::Greater),
            x if x < hand_value_2 => Some(Ordering::Less),
            x => match x {
                HAND_VALUE::Flush => {
                    for i in (0..=2).rev() {
                        if self.holder[i] > other.holder[i] {
                            return Some(Ordering::Greater);
                        }
                        if self.holder[i] < other.holder[i] {
                            return Some(Ordering::Less);
                        }
                    }

                    return Some(Ordering::Equal);
                }
                HAND_VALUE::ThreeOfAKind => {
                    return Some(Card::cmp(&self.holder[0], &other.holder[0]));
                }
                HAND_VALUE::Straight | HAND_VALUE::StraightFlush => {
                    for i in (0..=2).rev() {
                        let r = Card::cmp(&self.holder[i], &other.holder[i]);
                        if r != Ordering::Equal {
                            return Some(r);
                        }
                    }
                    return Some(Ordering::Equal);
                }
                HAND_VALUE::Pair => {
                    let comb_value_1 = get_a_pair(self);
                    let comb_value_2 = get_a_pair(other);

                    if comb_value_1.is_some() && comb_value_2.is_some() {
                        let comb1 = comb_value_1.unwrap();
                        let comb2 = comb_value_2.unwrap();

                        match comb1 {
                            x if x > comb2 => Some(Ordering::Greater),
                            x if x < comb2 => Some(Ordering::Less),
                            _ => Some(u8::cmp(
                                &get_highest_card_in_pair(self),
                                &get_highest_card_in_pair(other),
                            )),
                        };
                    }
                    None
                }
                _ => Some(Ordering::Equal),
            },
        };

        //Pairs

        None
    }
}
