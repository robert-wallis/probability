use rand::prelude::*;
mod stats;
mod opposite;
mod control;
mod flipper;
use stats::Stats;


fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut last_result: bool = false;
    let mut run: i32 = 0;
    let mut longest_run: i32 = 0;

    let total_tries = 1_000_000;
    let mut pred_control = control::Prediction::new();
    let mut pred_control_stats = Stats(0);
    let mut pred_flipper = flipper::Prediction::new();
    let mut pred_flipper_stats = Stats(0);
    let mut pred_opposite = opposite::Prediction::new();
    let mut pred_opposite_stats = Stats(0);

    for _ in 0..total_tries {
        let result: bool = rng.gen_bool(0.5);

        if result == last_result {
            run += 1;
        } else {
            run = 1;
            last_result = result;
        }
        if run > longest_run {
            longest_run = run;
        }

        if pred_control.predict() == result {
            pred_control_stats.0 += 1;
        }
        if pred_flipper.predict() == result {
            pred_flipper_stats.0 += 1;
        }
        if pred_opposite.predict(result) == result {
            pred_opposite_stats.0 += 1;
        }
    }
    println!("total runs: {}", total_tries);
    println!("longest run: {}", longest_run);
    println!("control: {}", pred_control_stats.score(total_tries));
    println!("flipper: {}", pred_flipper_stats.score(total_tries));
    println!("opposite: {}", pred_opposite_stats.score(total_tries));
}

