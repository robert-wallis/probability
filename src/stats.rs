pub struct Stats {
    pub correct: u32,
}

impl Stats {
    pub fn new() -> Stats {
        Stats { correct: 0 }
    }
    pub fn accuracy(&self, total_tries: u32) -> f32 {
        self.correct as f32 / total_tries as f32
    }
}
