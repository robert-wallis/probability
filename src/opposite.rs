use crate::{app_state::AppState, predictor::Predictor};
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

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Opposite")
    }
}
