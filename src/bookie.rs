use std::fmt;

use crate::app_state::AppState;

#[derive(Default)]
pub struct Bookie {
    money: u32,
    bet: Bet,
}

#[derive(Default, Clone, Copy)]
pub struct Bet {
    pub wager: u32,
    pub on: bool,
}

impl Bookie {
    pub fn new(balance: u32) -> Bookie {
        Bookie {
            money: balance,
            bet: Bet::default(),
        }
    }

    // place a bet on `on` to be correct, for `wager` amount
    pub fn bet(&mut self, bet: &Bet) {
        self.bet.wager = if bet.wager > self.money {
            self.money
        } else {
            bet.wager
        };
        self.bet.on = bet.on;
    }
    // do accounting on account, win wager if correct, loose wager if incorrect, reset wager to 0
    pub fn result(&mut self, winner: bool) {
        if self.bet.on == winner {
            self.money += self.bet.wager;
        } else {
            self.money -= self.bet.wager;
        }
        self.bet.wager = 0;
    }
    pub fn deposit(&mut self, amount: u32) {
        self.money = amount;
    }
    // get the current balance
    pub fn get_balance(&self) -> u32 {
        self.money
    }
}

impl fmt::Display for Bookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "money: {}", self.money)
    }
}

pub trait Better {
    fn bet(&mut self, state: &AppState) -> Option<Bet>;
}
