use crate::{bookie::Bookie, predictor::Predictor, stats::Stats};
pub struct Runner {
    pub predictor: Box<dyn Predictor>,
    pub stats: Stats,
    pub bookie: Bookie,
}

#[macro_export]
macro_rules! runner {
    ( $predictor:expr, $total:expr ) => {
        Runner {
            predictor: Box::new($predictor),
            stats: Stats::default(),
            bookie: Bookie::new($total),
        }
    };
}
