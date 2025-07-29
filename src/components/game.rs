use std::cell::OnceCell;

use crate::components::{deck::Deck, strategy::{Action, Strategy}};

pub struct Game {
    balance: i128,
    deck: Deck,
    strat: Strategy,
    played: bool,
    outcome: i128,
    next_action: OnceCell<Action>
}