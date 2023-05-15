use super::{
    account::Account,
    app_state::AppState,
    predictor::Predictor,
    stats::{FinalStats, RunningStats},
};
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

pub trait RunnerLoop {
    fn each_run(
        &mut self,
        name: &str,
        state: &AppState,
        final_stats: &FinalStats,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
