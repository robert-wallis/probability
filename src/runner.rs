use crate::{account::Account, predictor::Predictor, stats::RunningStats};
pub struct Runner {
    pub predictor: Box<dyn Predictor>,
    pub stats: RunningStats,
    pub account: Account,
}

#[macro_export]
macro_rules! runner {
    ( $predictor:expr, $total:expr ) => {
        Runner {
            predictor: Box::new($predictor),
            stats: RunningStats::default(),
            account: Account::new($total),
        }
    };
}
