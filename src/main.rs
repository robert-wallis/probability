use rand::prelude::*;
mod app_state;
pub mod bookie;
mod control;
mod flipper;
mod money;
mod opposite;
mod predictor;
mod stats;

use crate::{bookie::Bookie, predictor::Predictor, stats::Stats};

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut state = app_state::AppState::new(1_000_000);

    struct Runner {
        predictor: Box<dyn Predictor>,
        stats: Stats,
        bookie: Bookie,
    }

    let mut runners: Vec<Runner> = vec![
        Runner {
            predictor: Box::new(control::Prediction::new()),
            stats: Stats::default(),
            bookie: Bookie::new(state.total_count),
        },
        Runner {
            predictor: Box::new(flipper::Prediction::new()),
            stats: Stats::default(),
            bookie: Bookie::new(state.total_count),
        },
        Runner {
            predictor: Box::new(opposite::Prediction::new()),
            stats: Stats::default(),
            bookie: Bookie::new(state.total_count),
        },
        Runner {
            predictor: Box::new(money::Prediction::new()),
            stats: Stats::default(),
            bookie: Bookie::new(state.total_count),
        },
    ];

    for _ in 0..state.total_count {
        let throw: bool = rng.gen_bool(0.5);

        for runner in &mut runners {
            let prediction = runner.predictor.predict(&state);
            runner.predictor.bet(&mut runner.bookie, &state);
            runner.bookie.result(throw);
            runner.stats.update(prediction == throw);
        }

        state = state.next(throw);
    }
    println!("total runs: {}", state.total_count);
    println!("longest run: {}", state.longest_run);

    for runner in &runners {
        println!(
            "{}: {}, {}",
            runner.predictor,
            runner.stats.accuracy(state.total_count),
            runner.bookie,
        );
    }
}
