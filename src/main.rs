use rand::prelude::*;
mod app_state;
mod control;
mod flipper;
mod money;
mod opposite;
mod predictor;
mod stats;

use crate::{predictor::Predictor, stats::Stats};

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut state = app_state::AppState::new(1_000_000);
    let mut runners: Vec<(Box<dyn Predictor>, Stats)> = vec![
        (Box::new(control::Prediction::new()), Stats::default()),
        (Box::new(flipper::Prediction::new()), Stats::default()),
        (Box::new(opposite::Prediction::new()), Stats::default()),
        (
            Box::new(money::Prediction::new(state.total_count)),
            Stats::default(),
        ),
    ];

    for _ in 0..state.total_count {
        let result: bool = rng.gen_bool(0.5);

        for runner in &mut runners {
            let correct = runner.0.predict(&state) == result;

            runner.1.update(correct);
        }

        state = state.next(result);
    }
    println!("total runs: {}", state.total_count);
    println!("longest run: {}", state.longest_run);

    for runner in &runners {
        println!("{}: {}", runner.0, runner.1.accuracy(state.total_count));
    }
}
