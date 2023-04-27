use crate::app_state::AppState;
use std::fmt;

pub trait Predictor: fmt::Display {
    fn predict(&mut self, _: &AppState) -> bool;
}
