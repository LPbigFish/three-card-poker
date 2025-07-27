use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SUIT {
    SPADES,
    CLUBS,
    HEARTS,
    DIAMONDS
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
    TWO = 2
}

#[derive(Clone, Copy)]
pub struct Card {
    pub value: u8,
    pub suit: SUIT,
}

impl Card {
    pub fn new( value: u8, suit: SUIT ) -> Self {
        Card { suit: suit, value: value }
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
    
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if other < self { self } else { other }
    }
    
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if other < self { other } else { self }
    }
    
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}