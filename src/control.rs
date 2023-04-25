use rand::prelude::*;

use crate::{app_state::AppState, predictor::Predictor};

pub struct Prediction {
    rng: ThreadRng,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction {
            rng: rand::thread_rng(),
        }
    }
}

impl Predictor for Prediction {
    fn predict(&mut self, _: &AppState) -> bool {
        self.rng.gen_bool(0.5)
    }
}
