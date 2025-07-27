use crate::components::{card::{Card, CardValue, SUIT}, hand::Hand};

#[derive(Clone, Copy)]
pub enum Action {
    Raise2x,
    Lower2x,
    BackToBase,
    None,
}

pub struct Strategy {
    ante: i32,
    max_ante: i32,
    on_win: Action,
    on_loss: Action,
    minimal_playable_hand: Hand,
}

impl Default for Strategy {
    fn default() -> Self {
        Self {
            ante: 1,
            max_ante: 5,
            on_win: Action::None,
            on_loss: Action::None,
            minimal_playable_hand: Hand::new([
                Card::new(CardValue::QUEEN as u8, SUIT::SPADES),
                Card::new(CardValue::SEVEN as u8, SUIT::CLUBS),
                Card::new(CardValue::FIVE as u8, SUIT::DIAMONDS),
            ]),
        }
    }
}

impl Strategy {
    pub fn new(ante: i32, max_ante: i32) -> Self {
        Self {
            ante: ante,
            max_ante: max_ante,
            on_win: Action::None,
            on_loss: Action::None,
            minimal_playable_hand: Hand::default(),
        }
    }

    pub fn set_on_win(&self, on_win: Action) -> Self {
        Self {
            ante: self.ante,
            max_ante: self.max_ante,
            on_win,
            on_loss: self.on_loss,
            minimal_playable_hand: self.minimal_playable_hand.clone(),
        }
    }

    pub fn set_on_loss(&self, on_loss: Action) -> Self {
        Self {
            ante: self.ante,
            max_ante: self.max_ante,
            on_win: self.on_win,
            on_loss,
            minimal_playable_hand: self.minimal_playable_hand.clone(),
        }
    }

    pub fn set_minimal_playable_hand(&self, hand: Hand) -> Self {
        Self {
            ante: self.ante,
            max_ante: self.max_ante,
            on_win: self.on_win,
            on_loss: self.on_loss,
            minimal_playable_hand: hand,
        }
    }
    
    pub fn ante(&self) -> i32 {
        self.ante
    }
    
    pub fn max_ante(&self) -> i32 {
        self.max_ante
    }
    
    pub fn on_win(&self) -> Action {
        self.on_win
    }
    
    pub fn on_loss(&self) -> Action {
        self.on_loss
    }
    
    pub fn minimal_playable_hand(&self) -> &Hand {
        &self.minimal_playable_hand
    }
}
