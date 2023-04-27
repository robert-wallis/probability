use rand::prelude::*;
mod app_state;
mod control;
mod flipper;
mod money;
mod opposite;
mod predictor;
mod stats;

use crate::predictor::Predictor;

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut state = app_state::AppState::new(1_000_000);

    let mut runners: [Box<dyn Predictor>; 4] = [
        Box::new(control::Prediction::new()),
        Box::new(flipper::Prediction::new()),
        Box::new(opposite::Prediction::new()),
        Box::new(money::Prediction::new(state.total_count)),
    ];

    for _ in 0..state.total_count {
        let result: bool = rng.gen_bool(0.5);

        for runner in &mut runners {
            let correct = runner.predict(&state) == result;
            runner.update_stats(correct);
        }

        state = state.next(result);
    }
    println!("total runs: {}", state.total_count);
    println!("longest run: {}", state.longest_run);

    for runner in &runners {
        println!("{}: {}", runner, runner.accuracy(state.total_count));
    }
}
