use std::fmt;

use crate::{
    account::{Bet, Better},
    app_state::AppState,
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
        let wager = 2 * state.current_run;
        Some(Bet {
            wager,
            on: self.guess,
        })
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Money")
    }
}
