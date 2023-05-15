use super::{app_state::AppState, runner::RunnerLoop, stats::FinalStats};

pub fn print(name: &str, state: &AppState, final_stats: &FinalStats) {
    println!("{}", state);
    println!("* {:10} {}", name, final_stats,);
}

pub struct IO;

impl RunnerLoop for IO {
    fn each_app(&self, state: &AppState) {
        println!("{}", state);
    }

    fn each_run(
        &mut self,
        name: &str,
        state: &AppState,
        final_stats: &FinalStats,
    ) -> Result<(), Box<dyn std::error::Error>> {
        print(name, state, final_stats);
        Ok(())
    }
}
