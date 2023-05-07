use rand::prelude::*;
use std::error::Error;

use crate::{
    flipper::{
        app_state::AppState,
        predictors::{control, flipper, money, opposite},
        runner::Runner,
    },
    runner,
};

pub fn app(total_count: u32) -> Result<(AppState, Vec<Runner>), Box<dyn Error>> {
    let mut rng: ThreadRng = rand::thread_rng();
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

    Ok((state, runners))
}
