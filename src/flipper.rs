
pub struct Prediction {
    prediction: bool,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction { prediction: true }
    }

    pub fn predict(&mut self) -> bool {
        self.prediction = !self.prediction;
        self.prediction
    }
}

