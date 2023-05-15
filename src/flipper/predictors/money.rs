use std::fmt;

use crate::flipper::{
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

#[cfg(test)]
mod tests {
    use super::{AppState, Better, Prediction, Predictor};

    #[test]
    fn chooses_the_opposite() {
        // GIVEN the money predictor
        let mut p = Prediction::new();
        let mut app_state = AppState::new(0);

        // WHEN predict is called
        app_state = app_state.next(true);
        let first = p.predict(&app_state);

        app_state = app_state.next(false);
        let second = p.predict(&app_state);

        // THEN it should predict the opposite
        assert!(!first, "first should be false");
        assert!(second, "second should be true");
    }

    #[test]
    fn bets_double_down_every_other_time() {
        // GIVEN the money predictor
        let mut p = Prediction::new();
        let mut app_state = AppState::new(0);

        // WHEN bet is called
        app_state = app_state.next(false);
        let first = p.bet(&app_state);
        app_state = app_state.next(false);
        let second = p.bet(&app_state);
        app_state = app_state.next(false);
        let third = p.bet(&app_state);
        app_state = app_state.next(true);
        let fourth = p.bet(&app_state);

        // THEN it should bet double down every other time
        assert!(
            matches!(first, None),
            "first bet should be nothing, no prior"
        );
        // TODO: bet should be 2, it's the first time it's seen a run
        assert_eq!(
            second.expect("second should be a bet").wager,
            4,
            "second bet should be 4"
        );
        // TODO: bet should be 4
        assert_eq!(
            third.expect("third should be a bet").wager,
            6,
            "third bet should be 6"
        );
        assert!(
            matches!(fourth, None),
            "fourth bet should be nothing, no prior"
        );
    }

    #[test]
    fn name() {
        assert_eq!(
            "Money".to_string(),
            Prediction::new().to_string(),
            "name is Money"
        );
    }
}
