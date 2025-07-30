use std::fmt::{Display, Write};

use crate::components::{
    deck::Deck,
    hand::HandValue,
    strategy::{Action, Strategy},
};

pub struct GameResult {
    deck: Deck,
    played: bool,
    dealer_qualified: bool,
    player_won: bool,
    outcome: f32,
    ante_bet: f32,
    next_action: Action,
}

impl GameResult {
    fn new(deck: Deck, strat: &Strategy, action: Action) -> Self {
        let next_action;

        let ante = strat.ante()
            * match action {
                Action::BackToBase => 1.0,
                Action::Lower2x => 0.5,
                Action::Raise2x => 2.0,
                Action::None => 1.0,
            };

        let (p, d) = deck.deal_both();
        let played = p > *strat.minimal_playable_hand();

        let res = if played { p > d } else { false };

        let qual = d.key() > (HandValue::HighCard, 12, 0, 0);

        let out = if played {
            if res {
                let mult = match p.get_hand_value() {
                    HandValue::RoyalFlush | HandValue::StraightFlush => 5.0,
                    HandValue::ThreeOfAKind => 3.0,
                    _ => 1.0,
                };

                next_action = strat.on_win();

                mult * ante + ante * 2.0
            } else {
                next_action = strat.on_loss();
                -2.0 * ante
            }
        } else {
            next_action = strat.on_loss();
            -ante
        };

        Self {
            deck: deck.clone(),
            played: played,
            dealer_qualified: qual,
            player_won: res,
            outcome: out,
            ante_bet: ante,
            next_action: next_action,
        }
    }
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (p, d) = self.deck.deal_both();

        todo!()
    }
}
