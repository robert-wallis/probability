use rand::prelude::*;
pub mod account;
mod app_state;
mod predictor;
#[macro_use]
mod runner;
mod predictors;
mod stats;

use crate::{
    account::Account,
    predictors::*,
    runner::Runner,
    stats::{FinalStats, RunningStats},
};

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let total_count: u32 = 1_000_000;
    let mut state = app_state::AppState::new(total_count);

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
    println!("{}", state);

    for runner in &runners {
        println!(
            "* {:10} {}",
            runner.predictor.to_string(),
            FinalStats::new(&runner.stats, &runner.account, state.total_count,),
        );
    }
}
