use crate::app_state::AppState;
use std::fmt;

pub trait Predictor: fmt::Display {
    fn predict(&mut self, _: &AppState) -> bool;
    fn accuracy(&self, total_tries: u32) -> f32;
    fn update_stats(&mut self, correct: bool);
}
