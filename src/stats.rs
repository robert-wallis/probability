pub struct Stats(pub i32);

impl Stats {
    pub fn score(&self, total_tries: i32) -> f32 {
        self.0 as f32 / total_tries as f32
    }
}