use std::cell::{RefCell, Cell};
use crate::account;
use crate::account::Account;
use crate::transaction::Transaction;

struct Bank {
    accounts_count: Cell<isize>,
    accounts: RefCell<Vec<Account>>,
}

unsafe impl Sync for Bank {}

impl Bank {
    pub fn new() -> Self {
        Bank { accounts_count: Cell::new(0), accounts: RefCell::new(vec![]) }
    }

    pub fn create_account(&self) -> String {
        let account_number = self.generate_account_number();
        self.accounts.borrow_mut().push(Account::new(account_number.clone()));
        account_number
    }

    pub fn deposit_cash(&self, account_number: &str, amount: usize) -> Result<(), String> {
        self.accounts
            .borrow_mut()
            .iter()
            .find(|account| account.has_account_number(account_number))
            .map(|account| {
                account.perform_transaction(Transaction::credit_of_amount(amount)).unwrap();
                Ok(())
            })
            .unwrap_or(Err("Account not found".to_string()))
    }

    pub fn check_balance(&self, account_number: &str) -> Option<usize> {
        self.accounts
            .borrow_mut()
            .iter()
            .find(|account| account.has_account_number(account_number))
            .map(|account| Some(account.get_balance()))
            .unwrap_or(None)
    }

    fn generate_account_number(&self) -> String {
        let accounts_count = self.accounts_count.get();
        self.accounts_count.set(accounts_count + 1);
        let account_number = (accounts_count + 1).to_string();
        account_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_an_account_and_gives_its_account_number() {
        let bank = Bank::new();
        let account_number_1 = bank.create_account();
        let account_number_2 = bank.create_account();

        assert_eq!(account_number_1, "1".to_string());
        assert_eq!(account_number_2, "2".to_string());
    }

    #[test]
    fn should_deposit_cash_in_given_account() {
        let bank = Bank::new();
        bank.create_account();
        let account_number = bank.create_account();
        bank.create_account();

        bank.deposit_cash(&account_number, 1400 as usize);
        assert_eq!(bank.check_balance(&account_number), Some(1400 as usize));
    }
}