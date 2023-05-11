use super::{app_state::AppState, account::Better};
use std::fmt;

pub trait Predictor: fmt::Display + Better {
    fn predict(&mut self, _: &AppState) -> bool;
}
