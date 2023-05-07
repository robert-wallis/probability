use crate::flipper::app_state::AppState;
use crate::flipper::bet::Bet;

pub trait Better {
    fn bet(&mut self, state: &AppState) -> Option<Bet>;
}
