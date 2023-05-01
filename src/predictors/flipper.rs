use crate::{
    account::{Bet, Better},
    app_state::AppState,
    predictor::Predictor,
};
use std::fmt;

pub struct Prediction {
    prediction: bool,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction { prediction: true }
    }
}

impl Predictor for Prediction {
    fn predict(&mut self, _: &AppState) -> bool {
        self.prediction = !self.prediction;
        self.prediction
    }
}

impl Better for Prediction {
    fn bet(&mut self, state: &AppState) -> Option<Bet> {
        let double_down = matches!(state.current_id % 4, 0 | 1); // double, double, single, single
        let wager = if double_down { 2 } else { 1 };
        Some(Bet {
            wager,
            on: self.prediction,
        })
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Flipper")
    }
}
