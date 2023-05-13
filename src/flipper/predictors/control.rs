use std::fmt;

use rand::{rngs::ThreadRng, Rng};

use crate::flipper::{
    account::{Bet, Better},
    app_state::AppState,
    predictor::Predictor,
};

pub struct Prediction {
    rng: ThreadRng,
    guess: bool,
}

/// guesses and bets randomly
impl Prediction {
    pub fn new() -> Prediction {
        Prediction {
            rng: rand::thread_rng(),
            guess: false,
        }
    }
}

/// guess randomly by flipping another coin (rng)
impl Predictor for Prediction {
    fn predict(&mut self, _: &AppState) -> bool {
        self.guess = self.rng.gen_bool(0.5);
        self.guess
    }
}

/// doubles the bet 50% of the time, because it's a 50% probabiliy that it's the same as last time
impl Better for Prediction {
    fn bet(&mut self, _state: &AppState) -> Option<Bet> {
        let double_down = self.rng.gen_bool(0.5);
        Some(Bet {
            wager: if double_down { 2 } else { 1 },
            on: self.guess,
        })
    }
}

/// the name of the predictor is "Control"
impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Control")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn predictions_and_bets() {
        // GIVEN a predictor
        let runs = 10000;
        let mut p = Prediction::new();
        let state = AppState::new(runs);

        // it should predict about 50%
        let mut true_count = 0;
        let mut bet_on_true_count = 0;
        let mut double_down_count = 0;
        for _ in 0..runs {
            if p.predict(&state) {
                true_count += 1;
            }
            if let Some(b) = p.bet(&state) {
                if b.on {
                    bet_on_true_count += 1;
                }
                if b.wager == 2 {
                    double_down_count += 1;
                }
            }
        }

        assert!(
            true_count > 4900 && true_count < 5100,
            "true_count:{} should be about 50% of {}",
            true_count,
            runs
        );
        assert!(
            bet_on_true_count == true_count,
            "bet_on_true_count:{} should be exactly what it guessed true_count:{}",
            bet_on_true_count,
            true_count
        );
        assert!(
            double_down_count > 4900 && double_down_count < 5100,
            "double_down_count:{} should be about 50% of {}",
            double_down_count,
            runs
        );
    }

    #[test]
    fn name() {
        let p = Prediction::new();
        assert_eq!("Control", p.to_string());
    }
}
