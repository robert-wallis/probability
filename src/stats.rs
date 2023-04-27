#[derive(Clone, Default)]
pub struct Stats {
    correct: u32,
}

impl Stats {
    pub fn accuracy(&self, total_tries: u32) -> f32 {
        self.correct as f32 / total_tries as f32
    }
    pub fn update(&mut self, correct: bool) {
        self.correct += if correct { 1 } else { 0 }
    }
}
