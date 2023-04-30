use crate::{bookie::Bookie, predictor::Predictor, stats::RunningStats};
pub struct Runner {
    pub predictor: Box<dyn Predictor>,
    pub stats: RunningStats,
    pub bookie: Bookie,
}

#[macro_export]
macro_rules! runner {
    ( $predictor:expr, $total:expr ) => {
        Runner {
            predictor: Box::new($predictor),
            stats: RunningStats::default(),
            bookie: Bookie::new($total),
        }
    };
}
