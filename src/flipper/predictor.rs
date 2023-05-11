use std::fmt;

use super::{account::Better, app_state::AppState};

pub trait Predictor: fmt::Display + Better {
    fn predict(&mut self, _: &AppState) -> bool;
}
