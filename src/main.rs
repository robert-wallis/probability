use rand::prelude::*;
mod app_state;
mod control;
mod flipper;
mod money;
mod opposite;
mod predictor;
mod stats;
use stats::Stats;

use crate::predictor::Predictor;

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut state = app_state::AppState::new(1_000_000);

    struct Runner {
        predictor: Box<dyn Predictor>,
        stats: Stats,
    }

    let mut runners = [
        Runner {
            predictor: Box::new(control::Prediction::new()),
            stats: Stats::new(),
        },
        Runner {
            predictor: Box::new(flipper::Prediction::new()),
            stats: Stats::new(),
        },
        Runner {
            predictor: Box::new(opposite::Prediction::new()),
            stats: Stats::new(),
        },
        Runner {
            predictor: Box::new(money::Prediction::new(state.total_count)),
            stats: Stats::new(),
        },
    ];

    for _ in 0..state.total_count {
        let result: bool = rng.gen_bool(0.5);

        for runner in &mut runners {
            if runner.predictor.predict(&state) == result {
                runner.stats.correct += 1;
            }
        }

        state = state.next(result);
    }
    println!("total runs: {}", state.total_count);
    println!("longest run: {}", state.longest_run);

    for runner in &runners {
        println!(
            "{}: {}",
            runner.predictor,
            runner.stats.accuracy(state.total_count)
        );
    }
}
