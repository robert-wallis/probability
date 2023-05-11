use std::collections::BTreeMap;
use std::fmt;

// information about the current random generation
pub struct AppState {
    pub last_result: bool,               // last roll
    pub current_result: bool,            // current roll
    pub current_run: u32,                // current streak of the same result
    pub longest_run: u32,                // largest streak of the same result
    pub run_lengths: BTreeMap<u32, u32>, // <length, times_occurred>
    pub count_result_in_a_run: u32,      // how many results were in runs
    pub total_count: u32,                // total number of rolls
    pub current_id: u32,                 // the current run index x of `total_count`
}

impl AppState {
    pub fn new(total_count: u32) -> AppState {
        AppState {
            last_result: false,
            current_result: false,
            current_run: 0,
            longest_run: 0,
            run_lengths: BTreeMap::new(),
            count_result_in_a_run: 0,
            total_count,
            current_id: 0,
        }
    }

    pub fn next(mut self, result: bool) -> AppState {
        let is_run = result == self.last_result;
        let current_run = if is_run { self.current_run + 1 } else { 1 };
        if !is_run {
            self.run_lengths
                .entry(self.current_run)
                .and_modify(|w| *w += 1)
                .or_insert(1);
        }
        let count_result_in_a_run = self.count_result_in_a_run + if is_run { 1 } else { 0 };
        AppState {
            last_result: self.current_result,
            current_result: result,
            current_run,
            longest_run: std::cmp::max(current_run, self.longest_run),
            run_lengths: self.run_lengths,
            count_result_in_a_run,
            total_count: self.total_count,
            current_id: self.current_id + 1,
        }
    }
}

impl fmt::Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "total rolls: {}", self.total_count)?;
        writeln!(
            f,
            "total runs: {} ({}%)",
            self.count_result_in_a_run,
            self.count_result_in_a_run as f32 / self.total_count as f32
        )?;
        writeln!(f, "longest run: {}", self.longest_run)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let a = AppState::new(1);
        assert_eq!("total rolls: 1\ntotal runs: 0 (0%)\nlongest run: 0\n", a.to_string());
    }
}