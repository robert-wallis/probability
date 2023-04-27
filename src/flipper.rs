use crate::{
    app_state::AppState,
    bookie::{Bet, Better, Bookie},
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
    fn bet(&mut self, bookie: &mut Bookie, _state: &AppState) {
        bookie.bet(&Bet {
            wager: 1,
            on: self.prediction,
        });
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Flipper")
    }
}
