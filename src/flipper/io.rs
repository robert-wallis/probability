use super::{app_state::AppState, runner::RunnerLoop, stats::FinalStats};

pub struct IO<'w> {
    writer: &'w mut dyn std::io::Write,
}

impl<'w> IO<'w> {
    pub fn new(writer: &'w mut dyn std::io::Write) -> Self {
        Self { writer }
    }
}

impl RunnerLoop for IO<'_> {
    fn each_run(
        &mut self,
        name: &str,
        state: &AppState,
        final_stats: &FinalStats,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.writer, "{}", state)?;
        writeln!(self.writer, "* {:10} {}", name, final_stats)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AppState, FinalStats, RunnerLoop, IO};
    use std::io::BufWriter;

    #[test]
    fn prints() {
        // GIVEN a IO object
        let mut buffer = BufWriter::new(vec![]);
        // block for io to release buffer
        {
            let mut io = IO::new(&mut buffer);
            let name = "test";
            let state = AppState::new(100);
            let final_stats = FinalStats {
                money_difference: -3,
                expected_money: 100,
                accuracy: 0.5,
            };

            // WHEN a line is printed
            _ = io.each_run(name, &state, &final_stats);
        }

        // THEN it should write the line
        let string_of_buffer = bufwriter_to_string(buffer);
        assert_eq!(
            "total rolls: 100\n\
             total runs: 0 (0%)\n\
             longest run: 0\n\
             * test       accuracy:0.5      , $       -3:        -3\n",
            string_of_buffer,
            "a single line of status for a predictor"
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
