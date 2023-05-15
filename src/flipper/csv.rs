use super::{app_state::AppState, runner::RunnerLoop, stats::FinalStats};
use std::{error::Error, io};

pub struct Csv<'w> {
    writer: csv::Writer<&'w mut dyn io::Write>,
}

impl<'w> Csv<'w> {
    pub fn new(writer: &'w mut dyn io::Write) -> Self {
        let mut writer = csv::Writer::from_writer(writer);
        _ = writer.write_record([&"Kind", &"accuracy", &"money"]);
        Self { writer }
    }

    pub fn print(&mut self, name: &str, final_stats: &FinalStats) -> Result<(), Box<dyn Error>> {
        self.writer.write_record(&[
            name.to_string(),
            format!("{}", final_stats.accuracy),
            format!("{}", final_stats.money_difference),
        ])?;
        Ok(())
    }
}

impl RunnerLoop for Csv<'_> {
    fn each_app(&self, _state: &AppState) {}

    fn each_run(
        &mut self,
        name: &str,
        _state: &AppState,
        final_stats: &FinalStats,
    ) -> Result<(), Box<dyn Error>> {
        self.print(name, final_stats)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Csv;
    use std::io::BufWriter;

    #[test]
    fn runs_the_app_multiple_times() {
        // GIVEN multiple ocunts and multiple apps
        let mut buffer = BufWriter::new(vec![]);
        // WHEN csv is called
        _ = Csv::new(&mut buffer);

        // get the string from the BufWriter
        let string_of_buffer = String::from_utf8(
            buffer
                .into_inner()
                .expect("should be able to get the buffer"),
        )
        .expect("should be able to get the string");

        // THEN it should write the header
        assert_eq!(
            "Kind,accuracy,money\n", string_of_buffer,
            "total number of lines in the csv"
        );
    }
}
