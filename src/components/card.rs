use std::{
    cmp::Ordering,
    fmt::{Display, Write},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum SUIT {
    SPADES,
    CLUBS,
    HEARTS,
    DIAMONDS,
}

impl Display for SUIT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            SUIT::SPADES => '♠',
            SUIT::DIAMONDS => '♦',
            SUIT::HEARTS => '♥',
            SUIT::CLUBS => '♣',
        };

        f.write_char(char)
    }
}

pub enum CardValue {
    ACE = 14,
    KING = 13,
    QUEEN = 12,
    JACK = 11,
    TEN = 10,
    NINE = 9,
    EIGHT = 8,
    SEVEN = 7,
    SIX = 6,
    FIVE = 5,
    FOUR = 4,
    THREE = 3,
    TWO = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub value: u8,
    pub suit: SUIT,
}

impl Card {
    pub fn new(value: u8, suit: SUIT) -> Self {
        Card {
            suit: suit,
            value: value,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(res) = self.partial_cmp(other) {
            return res;
        }

        Ordering::Equal
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding = self.value.to_string();
        let value = match self.value {
            2_u8..=10_u8 => binding.as_str(),
            11 => "J",
            12 => "Q",
            13 => "K",
            14 => "A",
            _ => panic!("Wrong value found")
        };

        f.write_fmt(format_args!("{}{}", value, self.suit))
    }
}
