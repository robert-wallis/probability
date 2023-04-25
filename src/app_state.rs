// information about the current random generation
pub struct AppState {
    pub last_result: bool,
    pub current_result: bool,
    pub longest_run: u32,
    pub current_run: u32,
    pub total_count: u32, // total number of runs
    pub current_id: u32,  // the current run index x of `total_run_count`
}

impl AppState {
    pub fn new(total_count: u32) -> AppState {
        AppState {
            last_result: false,
            current_result: false,
            longest_run: 0,
            total_count,
            current_id: 0,
            current_run: 0,
        }
    }

    pub fn next(&self, result: bool) -> AppState {
        let run = if result == self.last_result {
            self.current_run + 1
        } else {
            1
        };
        AppState {
            last_result: self.current_result,
            current_result: result,
            longest_run: std::cmp::max(run, self.longest_run),
            current_run: run,
            total_count: self.total_count,
            current_id: self.current_id + 1,
        }
    }
}
