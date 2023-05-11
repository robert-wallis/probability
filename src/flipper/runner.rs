use super::{account::Account, predictor::Predictor, stats::RunningStats};
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
            stats: $crate::flipper::stats::RunningStats::default(),
            account: $crate::flipper::account::Account::new($total),
        }
    };
}
