use rand::prelude::*;
mod stats;
mod opposite;
mod control;
mod flipper;
mod app_state;
mod predictor;
use stats::Stats;

use crate::predictor::Predictor;


fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut state = app_state::AppState::new(1_000_000);

    struct Runner<'n> {
        name: &'n str,
        predictor: Box<dyn Predictor>,
        stats: Stats,
    }

    let mut runners = [
        Runner{name: "Control", predictor: Box::new(control::Prediction::new()), stats: Stats(0)},
        Runner{name: "Flipper", predictor: Box::new(flipper::Prediction::new()), stats: Stats(0)},
        Runner{name: "Opposite", predictor: Box::new(opposite::Prediction::new()), stats: Stats(0)},
    ];

    for _ in 0..state.total_count {
        let result: bool = rng.gen_bool(0.5);

        state = state.next(result);

        for runner in &mut runners {
            if runner.predictor.predict(&state) == result {
                runner.stats.0 += 1;
            }
        }
    }
    println!("total runs: {}", state.total_count);
    println!("longest run: {}", state.longest_run);

    for runner in &runners {
        println!("{}: {}", runner.name, runner.stats.score(state.total_count));
    }
}

