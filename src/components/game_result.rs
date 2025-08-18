use std::fmt::Display;

use crate::components::strategy::Action;

#[derive(Debug, Clone)]
pub struct GameResult {
    played: bool,
    dealer_qualified: bool,
    player_won: bool,
    outcome: f32,
    ante_bet: f32,
    next_action: Action,
}

impl GameResult {
    pub fn new(
        played: bool,
        dealer_qualified: bool,
        player_won: bool,
        outcome: f32,
        ante_bet: f32,
        next_action: Action,
    ) -> Self {
        GameResult {
            played,
            dealer_qualified,
            player_won,
            outcome,
            ante_bet,
            next_action,
        }
    }

    pub fn player_won(&self) -> bool {
        self.player_won
    }

    pub fn next_action(&self) -> Action {
        self.next_action
    }

    pub fn outcome(&self) -> f32 {
        self.outcome
    }

    pub fn ante_bet(&self) -> f32 {
        self.ante_bet
    }

    pub fn played(&self) -> bool {
        self.played
    }
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Ante: {:.2}\nPlayed: {}\nDealer qualified: {}\nOutcome: {:.2}\nPlayer won: {}",
            self.ante_bet, self.played, self.dealer_qualified, self.outcome, self.player_won
        ))
    }
}
