use rand::prelude::*;
mod app_state;
pub mod bookie;
mod predictor;
#[macro_use]
mod runner;
mod predictors;
mod stats;

use crate::{
    bookie::Bookie,
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
                runner.bookie.bet(&bet);
                runner.bookie.result(throw);
            }
            runner.stats.update(prediction == throw);
        }

        state = state.next(throw);
    }
    println!("total runs: {}", state.total_count);
    println!("longest run: {}", state.longest_run);

    for runner in &runners {
        println!(
            "* {:10} {:<30}",
            runner.predictor.to_string(),
            FinalStats::new(
                &runner.stats,
                &runner.bookie,
                state.total_count,
                state.total_count
            ),
        );
    }
}
