use super::{
    app_state::AppState,
    runner::{Runner, RunnerLoop},
    stats::FinalStats,
};

pub fn print(state: &AppState, runner: &Runner, final_stats: &FinalStats) {
    println!("{}", state);
    println!("* {:10} {}", runner.predictor.to_string(), final_stats,);
}

pub struct IO;

impl RunnerLoop for IO {
    fn each_app(&self, state: &AppState) {
        println!("{}", state);
    }

    fn each_run(
        &mut self,
        state: &AppState,
        runner: &Runner,
        final_stats: &FinalStats,
    ) -> Result<(), Box<dyn std::error::Error>> {
        print(state, runner, final_stats);
        Ok(())
    }
}
