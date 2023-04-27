use crate::{app_state::AppState, bookie::Better};
use std::fmt;

pub trait Predictor: fmt::Display + Better {
    fn predict(&mut self, _: &AppState) -> bool;
}
