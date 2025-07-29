use std::sync::LazyLock;

use rand::{Rng, seq::SliceRandom};

use crate::components::{
    card::{Card, SUIT},
    hand::Hand,
};

static BASE_DECK: LazyLock<[Card; 52]> = LazyLock::new(|| {
    use SUIT::*;
    std::array::from_fn(|i| {
        let suit = [SPADES, CLUBS, DIAMONDS, HEARTS][i / 13];
        let rank = 2u8 + (i % 13) as u8;
        Card::new(rank, suit)
    })
});

#[derive(Debug, Clone, Copy)]
pub struct Deck {
    order: [u8; 6],
}

impl Default for Deck {
    fn default() -> Self {
        Self { order: [0, 1, 2, 3, 4, 5] }
    }
}

impl Deck {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut order: [u8; 52] = std::array::from_fn(|i| i as u8);

        let (res, _) = order.partial_shuffle(rng, 6);

        let mut final_order: [u8; 6] = [0; 6];
        final_order.copy_from_slice(&res);
        
        Self { order: final_order }
    }

    fn player_hand(&self) -> Hand {
        Hand::new([
            BASE_DECK[self.order[0] as usize].clone(),
            BASE_DECK[self.order[1] as usize].clone(),
            BASE_DECK[self.order[2] as usize].clone(),
        ])
    }

    fn dealer_hand(&self) -> Hand {
        Hand::new([
            BASE_DECK[self.order[3] as usize].clone(),
            BASE_DECK[self.order[4] as usize].clone(),
            BASE_DECK[self.order[5] as usize].clone(),
        ])
    }

    pub fn deal_both(&self) -> (Hand, Hand) {
        (self.player_hand(), self.dealer_hand())
    }
}
