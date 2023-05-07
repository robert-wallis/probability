use crate::flipper::{app_state::AppState, better::Better};
use std::fmt;

pub trait Predictor: fmt::Display + Better {
    fn predict(&mut self, _: &AppState) -> bool;
}
