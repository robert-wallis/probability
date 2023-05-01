use rand::prelude::*;
use std::{error::Error, io, process};

pub mod account;
mod app_state;
mod predictor;
#[macro_use]
mod runner;
mod predictors;
mod stats;

use crate::{
    account::Account,
    app_state::AppState,
    predictors::*,
    runner::Runner,
    stats::{FinalStats, RunningStats},
};

fn main() {
    let total_count: u32 = 10_000;
    let total_app: u32 = 10_000;
    if let Err(e) = multi_csv(total_count, total_app) {
        println!("Error: {}", e);
        process::exit(1);
    }
}

fn multi_csv(total_count: u32, total_app: u32) -> Result<(), Box<dyn Error>> {
    let mut csv_writer = csv::Writer::from_writer(io::stdout());
    _ = csv_writer.write_record(&[&"Kind", &"accuracy", &"money"]);

    for _ in 0..total_app {
        match app(total_count) {
            Err(e) => {
                return Err(e);
            }
            Ok((state, runners)) => {
                for runner in runners {
                    let stats = FinalStats::new(&runner.stats, &runner.account, state.total_count);
                    csv_writer.write_record(&[
                        runner.predictor.to_string(),
                        format!("{}", stats.accuracy),
                        format!("{}", stats.money_difference),
                    ])?;
                }
                // print(&state, &runners);
            }
        }
    }
    Ok(())
}

fn app(total_count: u32) -> Result<(AppState, Vec<Runner>), Box<dyn Error>> {
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

fn print(state: &AppState, runners: &Vec<Runner>) {
    println!("{}", state);

    for runner in runners {
        println!(
            "* {:10} {}",
            runner.predictor.to_string(),
            FinalStats::new(&runner.stats, &runner.account, state.total_count,),
        );
    }
}
