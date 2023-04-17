use rand::prelude::*;

pub struct Prediction {
    rng: ThreadRng,
}

impl Prediction {
    pub fn new() -> Prediction {
        Prediction { rng: rand::thread_rng() }
    }

    pub fn predict(&mut self) -> bool {
        self.rng.gen_bool(0.5)
    }
}

