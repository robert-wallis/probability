use std::fmt;

use crate::{
    app_state::AppState,
    bookie::{Bet, Better, Bookie},
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
    fn bet(&mut self, bookie: &mut Bookie, state: &AppState) {
        let money = bookie.get_balance();
        if state.current_run <= 1 || money == 0 {
            return;
        }
        let gut_bet = 2 ^ (state.current_run - 1);
        let wager = if gut_bet <= money {
            gut_bet
        } else {
            money // all in
        };
        bookie.bet(&Bet {
            wager,
            on: self.guess,
        });
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Money")
    }
}
