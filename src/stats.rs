use std::fmt;

use crate::bookie::Bookie;

#[derive(Clone, Default)]
pub struct RunningStats {
    correct: u32,
}

impl RunningStats {
    pub fn accuracy(&self, total_tries: u32) -> f32 {
        self.correct as f32 / total_tries as f32
    }
    pub fn update(&mut self, correct: bool) {
        self.correct += if correct { 1 } else { 0 }
    }
}

pub struct FinalStats {
    money_difference: i32,
    accuracy: f32,
}

impl FinalStats {
    pub fn new(
        stats: &RunningStats,
        bookie: &Bookie,
        expected_money: u32,
        total_tries: u32,
    ) -> FinalStats {
        FinalStats {
            money_difference: bookie.get_balance() as i32 - expected_money as i32,
            accuracy: stats.accuracy(total_tries),
        }
    }
}

impl fmt::Display for FinalStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "accuracy:{:>9}, ${:>9}: {:>9}",
            self.accuracy,
            self.money_difference,
            self.money_difference - (self.accuracy * 2_000_000.0 - 1_000_000.0) as i32
        )
    }
}
