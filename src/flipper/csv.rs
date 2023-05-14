use super::{app::app, stats::FinalStats};
use std::{error::Error, io};

pub fn multi_csv(
    total_count: u32,
    total_app: u32,
    writer: &mut dyn io::Write,
) -> Result<(), Box<dyn Error>> {
    let mut csv_writer = csv::Writer::from_writer(writer);
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

#[cfg(test)]
mod tests {
    use super::multi_csv;
    use std::io::{BufRead, BufWriter};

    #[test]
    fn runs_the_app_multiple_times() {
        // GIVEN multiple ocunts and multiple apps
        let total_count = 2;
        let total_app = 3;
        let mut buffer = BufWriter::new(vec![]);
        // WHEN multi_csv is called
        let result = multi_csv(total_count, total_app, &mut buffer);

        // THEN it should run without error
        assert!(result.is_ok(), "multi_csv should run without error");

        // AND it should write the correct number of lines
        let number_of_predictors = 4;
        let total_line_items = total_app * number_of_predictors;
        let header_length = 1;
        let total_lines = total_line_items + header_length;
        assert_eq!(
            total_lines,
            buffer.into_inner().unwrap().lines().count() as u32,
            "total number of lines in the csv"
        );
    }
}
