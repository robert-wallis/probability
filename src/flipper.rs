use crate::{app_state::AppState, predictor::Predictor, stats::Stats};
use std::fmt;

pub struct Prediction {
    stats: Stats,
    prediction: bool,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction {
            stats: Stats::new(),
            prediction: true,
        }
    }
}

impl Predictor for Prediction {
    fn predict(&mut self, _: &AppState) -> bool {
        self.prediction = !self.prediction;
        self.prediction
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
        write!(f, "Flipper")
    }
}
