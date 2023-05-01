use crate::{account::Better, app_state::AppState};
use std::fmt;

pub trait Predictor: fmt::Display + Better {
    fn predict(&mut self, _: &AppState) -> bool;
}
