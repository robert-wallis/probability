
pub struct Prediction {
    prediction: bool,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction { prediction: true }
    }

    pub fn predict(&mut self, result: bool) -> bool {
        let guess = self.prediction;
        self.prediction = !result;
        guess
    }
}

