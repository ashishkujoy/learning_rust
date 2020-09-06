use std::cell::Cell;

struct Account {
    balance: Cell<usize>
}

unsafe impl Sync for Account {}

impl Account {
    fn new() -> Self {
        Account { balance: Cell::new(0) }
    }

    pub fn credit(&self, amount: usize) {
        let current_balance = self.balance.get();
        self.balance.set(current_balance + amount)
    }

    pub fn debit(&self, amount: usize) -> Result<(), String> {
        let current_balance = self.balance.get();
        if current_balance >= amount {
            self.balance.set(current_balance - amount);
            Ok(())
        } else {
            Err("Insufficient balance".to_string())
        }
    }

    pub fn get_balance(&self) -> usize {
        self.balance.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_credit_in_account() {
        let account = Account::new();
        account.credit(1000.45 as usize);
        assert_eq!(account.get_balance(), 1000.45 as usize)
    }

    #[test]
    fn should_debit_from_account() {
        let account = Account::new();
        account.credit(1000.45 as usize);
        account.debit(100 as usize).unwrap();
        assert_eq!(account.get_balance(), 900.45 as usize)
    }

    #[test]
    fn should_give_error_for_debit_on_insufficient_balance() {
        let account = Account::new();
        account.credit(1000.45 as usize);
        let error = account.debit(2000 as usize);

        assert_eq!(error, Err("Insufficient balance".to_string()));
    }
}