use crate::app_state::AppState;

pub trait Predictor {
	fn predict(&mut self, _: &AppState) -> bool;
}