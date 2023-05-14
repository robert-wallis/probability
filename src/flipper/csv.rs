use super::{app::app, stats::FinalStats};
use std::{error::Error, io};

pub fn multi_csv(total_count: u32, total_app: u32) -> Result<(), Box<dyn Error>> {
    let mut csv_writer = csv::Writer::from_writer(io::stdout());
    _ = csv_writer.write_record([&"Kind", &"accuracy", &"money"]);

    for _ in 0..total_app {
        let (state, runners) = app(total_count);
        for runner in runners {
            let stats = FinalStats::new(&runner.stats, &runner.account, state.total_count);
            csv_writer.write_record(&[
                runner.predictor.to_string(),
                format!("{}", stats.accuracy),
                format!("{}", stats.money_difference),
            ])?;
        }
    }
    Ok(())
}
