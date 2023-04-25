use crate::{app_state::AppState, predictor::Predictor};

pub struct Prediction {
    prediction: bool,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction { prediction: true }
    }
}

impl Predictor for Prediction {
    fn predict(&mut self, state: &AppState) -> bool {
        let guess = self.prediction;
        self.prediction = !state.current_result;
        guess
    }
}
