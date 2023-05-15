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
    use super::{AppState, Csv, FinalStats, RunnerLoop};
    use std::io::BufWriter;

    #[test]
    fn prints_header() {
        // GIVEN an io writer
        let mut buffer = BufWriter::new(vec![]);
        // WHEN a new Csv is made
        _ = Csv::new(&mut buffer);

        // THEN it should write the header
        let string_of_buffer = bufwriter_to_string(buffer);
        assert_eq!(
            "Kind,accuracy,money\n", string_of_buffer,
            "total number of lines in the csv"
        );
    }

    #[test]
    fn prints_csv_line() {
        // GIVEN a Csv object
        let mut buffer = BufWriter::new(vec![]);
        let app_state = AppState::new(100);

        // block to show Csv's ownership of buffer
        {
            let mut csv = Csv::new(&mut buffer);

            // WHEN Csv::print is called
            _ = csv.each_run(
                "name",
                &app_state,
                &FinalStats {
                    money_difference: -20,
                    expected_money: 100,
                    accuracy: 0.5,
                },
            );
        }

        // THEN it should print the correct lines
        let string_of_buffer = bufwriter_to_string(buffer);
        assert_eq!(
            "Kind,accuracy,money\nname,0.5,-20\n", string_of_buffer,
            "total number of lines in the csv"
        );
    }

    fn bufwriter_to_string(bufwriter: BufWriter<Vec<u8>>) -> String {
        String::from_utf8(
            bufwriter
                .into_inner()
                .expect("should be able to get the buffer"),
        )
        .expect("should be able to get the string")
    }
}
