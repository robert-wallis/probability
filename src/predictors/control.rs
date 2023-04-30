use rand::prelude::*;
use std::fmt;

use crate::{
    app_state::AppState,
    bookie::{Bet, Better},
    predictor::Predictor,
};

pub struct Prediction {
    rng: ThreadRng,
    guess: bool,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction {
            rng: rand::thread_rng(),
            guess: false,
        }
    }
}

impl Predictor for Prediction {
    fn predict(&mut self, _: &AppState) -> bool {
        self.guess = self.rng.gen_bool(0.5);
        self.guess
    }
}

impl Better for Prediction {
    fn bet(&mut self, _state: &AppState) -> Option<Bet> {
        Some(Bet {
            wager: 1,
            on: self.guess,
        })
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Control")
    }
}
