// information about the current random generation
pub struct AppState {
    pub last_result: bool,          // last roll
    pub current_result: bool,       // current roll
    pub current_run: u32,           // curent streak of the same result
    pub longest_run: u32,           // largest streak of the same result
    pub count_result_in_a_run: u32, // how many results were in runs
    pub total_count: u32,           // total number of rolls
    pub current_id: u32,            // the current run index x of `total_count`
}

impl AppState {
    pub fn new(total_count: u32) -> AppState {
        AppState {
            last_result: false,
            current_result: false,
            current_run: 0,
            longest_run: 0,
            count_result_in_a_run: 0,
            total_count,
            current_id: 0,
        }
    }

    pub fn next(&self, result: bool) -> AppState {
        let is_run = result == self.last_result;
        let current_run = if is_run { self.current_run + 1 } else { 1 };
        let count_result_in_a_run = self.count_result_in_a_run + if is_run { 1 } else { 0 };
        AppState {
            last_result: self.current_result,
            current_result: result,
            current_run,
            longest_run: std::cmp::max(current_run, self.longest_run),
            count_result_in_a_run,
            total_count: self.total_count,
            current_id: self.current_id + 1,
        }
    }
}
