use std::{cell::OnceCell, sync::LazyLock};

use rand::{Rng, seq::SliceRandom};

use crate::components::{
    card::{Card, CardValue, SUIT},
    game_result::GameResult,
    hand::{Hand, HandValue},
    strategy::{Action, Strategy},
};

static BASE_DECK: LazyLock<[Card; 52]> = LazyLock::new(|| {
    use SUIT::*;
    std::array::from_fn(|i| {
        let suit = [SPADES, CLUBS, DIAMONDS, HEARTS][i / 13];
        let rank = CardValue::array()[i % 13];
        Card::new(rank, suit)
    })
});

#[derive(Debug, Clone)]
pub struct Deck {
    order: [u8; 6],
    result: OnceCell<GameResult>,
}

impl Default for Deck {
    fn default() -> Self {
        Self {
            order: [0, 1, 2, 3, 4, 5],
            result: OnceCell::new(),
        }
    }
}

impl Deck {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut order: [u8; 52] = std::array::from_fn(|i| i as u8);

        let (res, _) = order.partial_shuffle(rng, 6);

        let mut final_order: [u8; 6] = [0; 6];
        final_order.copy_from_slice(&res);

        Self {
            order: final_order,
            result: OnceCell::new(),
        }
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

    pub fn get_game_result(&self, strat: Option<&Strategy>, action: Option<Action>) -> &GameResult {
        if self.result.get().is_none() && strat.is_some() && action.is_some() {
            self.result.get_or_init(|| {
                let strat = strat.unwrap();
                let action = action.unwrap();

                let next_action;

                let ante = strat.ante()
                    * match action {
                        Action::BackToBase => 1.0,
                        Action::Lower2x => 0.5,
                        Action::Raise2x => 2.0,
                        Action::None => 1.0,
                    };

                let (p, d) = self.deal_both();
                let played = p > *strat.minimal_playable_hand();

                let res = played && p > d;

                let qual = d.key() > (HandValue::HighCard, 12, 0, 0);

                let out = if played {
                    if res {
                        let mult = match p.get_hand_value() {
                            HandValue::RoyalFlush | HandValue::StraightFlush => 5.0,
                            HandValue::ThreeOfAKind => 3.0,
                            _ => 1.0,
                        };

                        next_action = strat.on_win();

                        (mult + if qual { 3.0 } else { 1.0 }) * ante
                    } else {
                        next_action = strat.on_loss();
                        -2.0 * ante
                    }
                } else {
                    next_action = strat.on_loss();
                    -ante
                };

                GameResult::new(played, qual, res, out, ante, next_action)
            });
        }

        if self.result.get().is_none() {
            panic!("Game result not initialized, ensure to call get_game_result with valid strategy and action.");
        } else {
            self.result.get().unwrap()
        }
    }
}
