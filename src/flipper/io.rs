use crate::flipper::{app_state::AppState, runner::Runner, stats::FinalStats};

pub fn print(state: &AppState, runners: &Vec<Runner>) {
    println!("{}", state);

    for runner in runners {
        println!(
            "* {:10} {}",
            runner.predictor.to_string(),
            FinalStats::new(&runner.stats, &runner.account, state.total_count,),
        );
    }
}
