use std::fmt;

use crate::{app_state::AppState, predictor::Predictor, stats::Stats};

pub struct Prediction {
    stats: Stats,
    money: u32,
    wager: u32,
    wager_prediction: bool,
}

impl Prediction {
    pub fn new(money: u32) -> Prediction {
        Prediction {
            stats: Stats::new(),
            money,
            wager: 0,
            wager_prediction: false,
        }
    }

    // calculate win/loss
    fn accounting(&mut self, state: &AppState) {
        if state.current_result == self.wager_prediction {
            self.money += self.wager;
        } else {
            self.money -= self.wager;
        }
    }

    fn bet(&mut self, state: &AppState) {
        self.accounting(state);
        if state.current_run <= 1 || self.money == 0 {
            self.wager = 0;
            return;
        }
        let gut_bet = 2 ^ (state.current_run - 1);
        if gut_bet <= self.money {
            self.wager = gut_bet;
            return;
        }
        self.wager = self.money; // all in
    }
}

impl Predictor for Prediction {
    fn predict(&mut self, state: &AppState) -> bool {
        if state.current_run > 1 {
            self.bet(state)
        }
        let guess = !state.current_result; // always guess the opposite
        self.wager_prediction = guess;
        guess
    }

    fn accuracy(&self, total_tries: u32) -> f32 {
        self.stats.accuracy(total_tries)
    }
    fn update_stats(&mut self, correct: bool) {
        self.stats.correct += if correct { 1 } else { 0 }
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Money: {}", self.money)
    }
}
