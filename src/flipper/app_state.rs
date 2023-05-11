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
        let last_result = self.current_result;
        let is_run = last_result == result;
        let current_run = if is_run { self.current_run + 1 } else { 1 };
        if !is_run {
            self.run_lengths
                .entry(self.current_run)
                .and_modify(|w| *w += 1)
                .or_insert(1);
        }
        let count_result_in_a_run = self.count_result_in_a_run + if is_run { 1 } else { 0 };
        AppState {
            last_result,
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
    fn next() {
        let count = 5;
        let start = AppState::new(count);

        let default_result = false;
        assert_eq!(count, start.total_count, "total_count");
        assert_eq!(0, start.current_run, "current_run");
        assert_eq!(0, start.longest_run, "longest_run");
        assert_eq!(0, start.count_result_in_a_run, "count_result_in_a_run");
        assert_eq!(0, start.current_id, "current_id");
        assert_eq!(default_result, start.last_result, "defaults to false");
        assert_eq!(default_result, start.current_result, "defaults to false");

        let state1_roll = true;
        let state1 = start.next(state1_roll);
        assert_eq!(
            default_result, state1.last_result,
            "copy from current_result of last run (default false)"
        );
        assert_eq!(state1_roll, state1.current_result, "current_result");
        assert_eq!(count, state1.total_count, "total_count");
        assert_eq!(1, state1.current_run, "current_run");
        assert_eq!(1, state1.longest_run, "longest_run");
        assert_eq!(
            0, state1.count_result_in_a_run,
            "count_result_in_a_run should be 0 since state changed"
        );
        assert_eq!(1, state1.current_id, "current_id");

        let state2_roll = true;
        let state2 = state1.next(state2_roll);
        assert_eq!(
            state1_roll, state2.last_result,
            "copy from current_result of last run"
        );
        assert_eq!(state2_roll, state2.current_result, "current_result");
        assert_eq!(count, state2.total_count, "total_count");
        assert_eq!(
            state1_roll, state2_roll,
            "same result as last time (sanity)"
        );
        assert_eq!(2, state2.current_run, "current_run");
        assert_eq!(2, state2.longest_run, "longest_run");
        assert_eq!(
            1, state2.count_result_in_a_run,
            "count_result_in_a_run now 1 because state was same for the first time"
        );
        assert_eq!(2, state2.current_id, "current_id");

        let state3_roll = false;
        let state3 = state2.next(state3_roll);
        assert_eq!(
            state2_roll, state3.last_result,
            "copy from current_result of last run"
        );
        assert_eq!(state3_roll, state3.current_result, "current_result");
        assert_eq!(count, state3.total_count, "total_count");
        assert_eq!(1, state3.current_run, "current_run");
        assert_eq!(2, state3.longest_run, "longest_run");
        assert_eq!(1, state3.count_result_in_a_run, "count_result_in_a_run");
        assert_eq!(3, state3.current_id, "current_id");

        let state4_roll = true;
        let state4 = state3.next(state4_roll);
        assert_eq!(
            state3_roll, state4.last_result,
            "copy from current_result of last run"
        );
        assert_eq!(state4_roll, state4.current_result, "current_result");
        assert_eq!(count, state4.total_count, "total_count");
        assert_eq!(1, state4.current_run, "current_run");
        assert_eq!(2, state4.longest_run, "longest_run");
        assert_eq!(1, state4.count_result_in_a_run, "count_result_in_a_run");
        assert_eq!(4, state4.current_id, "current_id");
    }

    #[test]
    fn display() {
        let a = AppState::new(1);
        assert_eq!(
            "total rolls: 1\ntotal runs: 0 (0%)\nlongest run: 0\n",
            a.to_string()
        );
    }
}
