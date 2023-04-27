use crate::{
    app_state::AppState,
    bookie::{Bet, Better, Bookie},
    predictor::Predictor,
};
use std::fmt;

pub struct Prediction;

impl Prediction {
    pub fn new() -> Prediction {
        Prediction
    }
}

impl Predictor for Prediction {
    fn predict(&mut self, state: &AppState) -> bool {
        !state.current_result
    }
}

impl Better for Prediction {
    fn bet(&mut self, bookie: &mut Bookie, state: &AppState) {
        bookie.bet(&Bet {
            wager: 1,
            on: !state.current_result,
        });
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Opposite")
    }
}
