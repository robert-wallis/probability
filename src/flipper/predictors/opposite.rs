use std::fmt;

use crate::flipper::{
    account::{Bet, Better},
    app_state::AppState,
    predictor::Predictor,
};

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
    fn bet(&mut self, state: &AppState) -> Option<Bet> {
        let double_down = matches!(state.current_id % 4, 0 | 1); // double, double, single, single
        let wager = if double_down { 2 } else { 1 };
        Some(Bet {
            wager,
            on: !state.current_result,
        })
    }
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Opposite")
    }
}

#[cfg(test)]
mod tests {
    use super::{AppState, Better, Prediction, Predictor};

    #[test]
    fn chooses_the_opposite() {
        // GIVEN the opposite predictor
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
        // GIVEN the opposite predictor
        let mut p = Prediction::new();
        let mut app_state = AppState::new(0);

        // WHEN bet is called
        let first = p.bet(&app_state).unwrap();
        app_state = app_state.next(true);
        let second = p.bet(&app_state).unwrap();
        app_state = app_state.next(false);
        let third = p.bet(&app_state).unwrap();
        app_state = app_state.next(true);
        let fourth = p.bet(&app_state).unwrap();

        // THEN it should bet double down every other time
        assert_eq!(first.wager, 2, "first bet should be double down");
        assert_eq!(second.wager, 2, "second bet should be double down");
        assert_eq!(third.wager, 1, "third bet should be single down");
        assert_eq!(fourth.wager, 1, "fourth bet should be single down");
    }

    #[test]
    fn name() {
        assert_eq!(
            "Opposite".to_string(),
            Prediction::new().to_string(),
            "name is Opposite"
        );
    }
}
