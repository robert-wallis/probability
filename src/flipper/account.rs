use std::fmt;

use super::app_state::AppState;

#[derive(Default)]
pub struct Account {
    money: u32,
    bet: Bet,
}

#[derive(Default, Clone, Copy)]
pub struct Bet {
    pub wager: u32,
    pub on: bool,
}

pub trait Bank {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
    fn get_balance(&self) -> u32;
}

pub trait Bookie {
    fn bet(&mut self, bet: &Bet);
    fn result(&mut self, winner: bool);
}

pub trait Better {
    fn bet(&mut self, state: &AppState) -> Option<Bet>;
}

impl Account {
    pub fn new(balance: u32) -> Account {
        Account {
            money: balance,
            bet: Bet::default(),
        }
    }
}

impl Bank for Account {
    fn deposit(&mut self, amount: u32) {
        self.money += amount;
    }

    fn withdraw(&mut self, amount: u32) {
        if self.money >= amount {
            self.money -= amount;
        } else {
            self.money = 0;
        }
    }

    fn get_balance(&self) -> u32 {
        self.money
    }
}

impl Bookie for Account {
    // place a bet on `on` to be correct, for `wager` amount
    fn bet(&mut self, bet: &Bet) {
        self.bet.wager = if bet.wager > self.money {
            self.money
        } else {
            bet.wager
        };
        self.bet.on = bet.on;
    }

    // do accounting on account, win wager if correct, loose wager if incorrect, reset wager to 0
    fn result(&mut self, winner: bool) {
        if self.bet.on == winner {
            self.deposit(self.bet.wager);
        } else {
            self.withdraw(self.bet.wager);
        }
        self.bet.wager = 0;
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "money: {}", self.money)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bookie() {
        let mut a = Account::new(100);
        assert_eq!(100, a.get_balance());

        a.bet(&Bet{wager:50, on: true});
        assert_eq!(50, a.bet.wager);
        assert!(a.bet.on);

        // no balance change yet
        assert_eq!(100, a.get_balance());

        // winner
        a.result(true);
        assert_eq!(150, a.get_balance());

        // no bets made
        assert_eq!(0, a.bet.wager);
        a.result(false);
        assert_eq!(0, a.bet.wager);
        assert_eq!(150, a.get_balance());
        a.result(true);
        assert_eq!(0, a.bet.wager);
        assert_eq!(150, a.get_balance());

        // loser
        a.bet(&Bet{wager:50, on: true});
        a.result(false);
        assert_eq!(100, a.get_balance());

        // over-bet
        a.bet(&Bet{wager:200, on: true});
        a.result(false);
        assert_eq!(0, a.get_balance());
    }

    #[test]
    fn bank() {
        let mut a = Account::new(100);

        // basic account setup
        assert_eq!(100, a.get_balance());

        // basic withdraw
        a.withdraw(50);
        assert_eq!(50, a.get_balance());

        // over withdraw
        a.withdraw(52);
        assert_eq!(0, a.get_balance());

        // basic deposit
        a.deposit(200);
        assert_eq!(200, a.get_balance());

        // == zero
        a.withdraw(200);
        assert_eq!(0, a.get_balance());

        // off by one
        a.deposit(37);
        a.withdraw(36);
        assert_eq!(1, a.get_balance());
    }


    #[test]
    fn display() {
        let a = Account::new(123);
        assert_eq!("money: 123", a.to_string());

        let b = Account::new(0);
        assert_eq!("money: 0", b.to_string());
    }

    #[test]
    fn one_hundred_percent_coverage() {
        let a = Account::default();
        assert_eq!(0, a.money);
        assert_eq!(0, a.bet.wager);
        assert!(!a.bet.on);
    }
}
