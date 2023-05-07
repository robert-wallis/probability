use std::fmt;

use crate::flipper::account::Account;

#[derive(Clone, Default)]
pub struct RunningStats {
    correct: u32,
    wrong: u32,
}

impl RunningStats {
    pub fn accuracy(&self) -> f32 {
        let correct = self.correct as f32;
        let wrong = self.wrong as f32;
        correct / (correct + wrong)
    }
    pub fn update(&mut self, correct: bool) {
        self.correct += if correct { 1 } else { 0 };
        self.wrong += if !correct { 1 } else { 0 };
    }
}

pub struct FinalStats {
    pub money_difference: i32,
    pub expected_money: u32,
    pub accuracy: f32,
}

impl FinalStats {
    pub fn new(stats: &RunningStats, account: &Account, expected_money: u32) -> FinalStats {
        FinalStats {
            money_difference: account.get_balance() as i32 - expected_money as i32,
            expected_money,
            accuracy: stats.accuracy(),
        }
    }
}

impl fmt::Display for FinalStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "accuracy:{:<9}, ${:>9}: {:>9}",
            self.accuracy,
            self.money_difference,
            self.money_difference
                - (self.accuracy * 2.0 * self.expected_money as f32 - self.expected_money as f32)
                    as i32
        )
    }
}
