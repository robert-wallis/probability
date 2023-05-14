use std::fmt;

use super::account::{Account, Bank};

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

#[cfg(test)]
mod tests {

    mod running_stats {
        use super::super::RunningStats;
        #[test]
        fn accuracy() {
            // GIVEN 50% correct answers
            let rs = RunningStats {
                correct: 2,
                wrong: 2,
            };
            // THEN accuracy is 50%
            assert_eq!(2.0 / 4.0, rs.accuracy(), "accuracy of 1/2 correct");
        }

        #[test]
        fn update() {
            // GIVEN some boolean values for results
            let results = [true, false, true, false, true];

            // WHEN we update the running stats
            let mut rs = RunningStats::default();
            for correct in results {
                rs.update(correct)
            }

            // THEN the running stats should be correct
            let total_true = results.iter().filter(|&x| *x).count() as u32;
            let total_false = results.iter().filter(|&x| !(*x)).count() as u32;

            assert_eq!(total_true, rs.correct, "total number of correct answers");
            assert_eq!(total_false, rs.wrong, "total number of correct answers");
        }
    }

    mod final_stats {
        use super::super::{FinalStats, RunningStats};
        use crate::flipper::account::Account;

        #[test]
        fn display() {
            // GIVEN a RunningStats and Account object
            // AND an expected money value
            let running_stats = RunningStats {
                correct: 2,
                wrong: 2,
            };
            let account = Account::new(100);
            let expected_money = 104;

            // WHEN the FinalStats object is created
            let final_stats = FinalStats::new(&running_stats, &account, expected_money);

            // THEN the display should be correct
            assert_eq!(
                "accuracy:0.5      , $       -4:        -4",
                final_stats.to_string(),
                "display of FinalStats"
            );
        }
    }
}
