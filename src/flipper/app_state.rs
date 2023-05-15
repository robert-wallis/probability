use std::collections::BTreeMap;
use std::fmt;

/// information about the current random generation
pub struct AppState {
    /// last roll
    pub last_result: bool,
    /// current roll
    pub current_result: bool,
    /// current streak of the same result
    pub current_run: u32,
    /// largest streak of the same result
    pub longest_run: u32,
    /// <length, times_occurred>
    pub run_lengths: BTreeMap<u32, u32>,
    /// how many results were in runs
    pub count_result_in_a_run: u32,
    /// total number of rolls
    pub total_count: u32,
    /// the current run index x of `total_count`
    pub current_id: u32,
}

impl AppState {
    /// create a new state
    /// total_count: the intended number of state changes
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

    /// moves self into a new state, calculating the internal sequence length values
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

/// formats the AppState to return for example:
/// > total rolls: 10
/// > total runs: 7, (0.7%)
/// > longest run: 4
impl fmt::Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "total rolls: {}", self.total_count)?;
        writeln!(
            f,
            "total runs: {} ({}%)",
            self.count_result_in_a_run,
            self.count_result_in_a_run as f32 / self.total_count as f32
        )?;
        write!(f, "longest run: {}", self.longest_run)
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
    fn next_saved_context() {
        // GIVEN a list of boolean values
        let list = [
            true, false, false, false, true, false, true, true, true, true, true, false, false,
            false, false, false, false, false, false, true, true, false, true, false,
        ];

        let mut state = AppState::new(list.len() as u32);

        // WHEN the next() function is called on that list
        for b in list {
            state = state.next(b);
        }

        // THEN the expected result should be whatever it was when I programmed it

        // programmatically determining these values
        let max_sequence = largest_sequence(&list);
        let list_last = *list.last().expect("bad test");
        assert_eq!(list_last, state.current_result, "current_result");
        assert_eq!(list.len() as u32, state.total_count, "total_count");
        assert_eq!(max_sequence as u32, state.longest_run, "longest_run");
        assert_eq!(list.len() as u32, state.current_id, "current_id");

        // eyeballing these values
        assert_eq!(1, state.current_run, "current_run");
        assert_eq!(14, state.count_result_in_a_run, "count_result_in_a_run");
    }

    #[test]
    fn display() {
        let a = AppState::new(1);
        assert_eq!(
            "total rolls: 1\ntotal runs: 0 (0%)\nlongest run: 0",
            a.to_string()
        );
    }

    /// Of the three correct implementations, the one that has the best readability is the following `largest_sequence`
    /// This implementation is straightforward and easy to understand. It uses a simple loop and conditional statements to keep track of the current and maximum sequences. The variable names (`max_sequence`, `current_sequence`, `previous_item`) are descriptive and help to clarify their purpose.
    /// The functional versions of the code, while concise, might be less readable for developers who are not familiar with functional programming concepts. They involve higher-order functions, closures, and methods like `fold`, `map`, and `unwrap_or`, which can introduce additional complexity for those who are new to these concepts.
    /// Therefore, in terms of readability, the initial imperative implementation with explicit loop and conditionals tends to be more approachable and easier to understand for most developers.
    /// -- Coding AI, 2023
    fn largest_sequence(list: &[bool]) -> usize {
        let mut max_sequence = 0;
        let mut current_sequence = 0;
        let mut previous_item: Option<bool> = None;

        for &item in list {
            if let Some(prev_item) = previous_item {
                if item != prev_item {
                    current_sequence = 0;
                }
                current_sequence += 1;
            }
            max_sequence = max_sequence.max(current_sequence);
            previous_item = Some(item);
        }

        max_sequence
    }

    #[test]
    fn test_largest_sequence() {
        assert_eq!(
            3,
            largest_sequence(&[true, false, false, false, true, true])
        );
        assert_eq!(
            4,
            largest_sequence(&[false, true, true, true, true, false, false, false, true, false])
        );
    }
}
