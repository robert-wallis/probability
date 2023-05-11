use std::fmt;

use rand::{Rng, rngs::ThreadRng};

use crate::flipper::{
    account::{Bet, Better},
    app_state::AppState,
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
        // about 50% of the time it will be the same roll as last time, so double down 50% of the
        // time
        let double_down = self.rng.gen_bool(0.5);
        Some(Bet {
            wager: if double_down { 2 } else { 1 },
            on: self.guess,
        })
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Control")
    }
}
