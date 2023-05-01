use std::fmt;

use crate::{
    app_state::AppState,
    account::{Bet, Better},
    predictor::Predictor,
};

pub struct Prediction {
    guess: bool,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction { guess: false }
    }
}

impl Predictor for Prediction {
    fn predict(&mut self, state: &AppState) -> bool {
        self.guess = !state.current_result;
        self.guess
    }
}

impl Better for Prediction {
    fn bet(&mut self, state: &AppState) -> Option<Bet> {
        if state.current_run <= 1 {
            return None;
        }
        let gut_bet = 2 ^ (state.current_run - 1);
        Some(Bet {
            wager: gut_bet,
            on: self.guess,
        })
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Money")
    }
}
