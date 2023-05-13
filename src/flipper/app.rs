use rand::prelude::*;

use crate::runner;

use super::{
    account::Bookie,
    app_state::AppState,
    predictors::{control, flipper, money, opposite},
    runner::Runner,
};

pub fn app(total_count: u32) -> (AppState, Vec<Runner>) {
    let mut rng: ThreadRng = thread_rng();
    let mut state = AppState::new(total_count);

    let mut runners: Vec<Runner> = vec![
        runner!(control::Prediction::new(), total_count),
        runner!(flipper::Prediction::new(), total_count),
        runner!(opposite::Prediction::new(), total_count),
        runner!(money::Prediction::new(), total_count),
    ];

    for _ in 0..state.total_count {
        let throw: bool = rng.gen_bool(0.5);

        for runner in &mut runners {
            let prediction = runner.predictor.predict(&state);
            let bet = runner.predictor.bet(&state);
            if let Some(bet) = bet {
                runner.account.bet(&bet);
                runner.account.result(throw);
            }
            runner.stats.update(prediction == throw);
        }

        state = state.next(throw);
    }

    (state, runners)
}
