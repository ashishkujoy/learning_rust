use std::cell::{Cell, RefCell};

use crate::account::Account;
use crate::transaction::Transaction;

struct Bank {
    accounts_count: Cell<isize>,
    accounts: RefCell<Vec<Account>>,
}

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
        self.perform_transaction(account_number, Transaction::credit_of_amount(amount))
    }

    pub fn withdraw_cash(&self, account_number: &str, amount: usize) -> Result<(), String> {
        self.perform_transaction(account_number, Transaction::debit_of_amount(amount))
    }

    pub fn check_balance(&self, account_number: &str) -> Option<usize> {
        self.map_account(account_number, Box::new(|account| { account.get_balance() }))
    }

    pub fn get_transactions(&self, account_number: &str) -> Option<Vec<Transaction>> {
        self.map_account(account_number, Box::new(|account| {
            account.transactions_history().clone()
        }))
    }

    fn map_account<T>(&self, account_number: &str, mapper: Box<dyn FnOnce(&Account) -> T>) -> Option<T> {
        self.accounts
            .borrow_mut()
            .iter()
            .find(|account| account.has_account_number(account_number))
            .map(mapper)
    }

    fn generate_account_number(&self) -> String {
        let accounts_count = self.accounts_count.get();
        self.accounts_count.set(accounts_count + 1);
        let account_number = (accounts_count + 1).to_string();
        account_number
    }

    fn perform_transaction(&self, account_number: &str, transaction: Transaction) -> Result<(), String> {
        self.map_account(account_number, Box::new(move |account| {
            account.perform_transaction(transaction)
        })).unwrap_or(Err("Account not found".to_string()))
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

        bank.deposit_cash(&account_number, 1400 as usize).unwrap();
        assert_eq!(bank.check_balance(&account_number), Some(1400 as usize));
    }

    #[test]
    fn should_debit_cash_from_given_account() {
        let bank = Bank::new();
        let account_number = bank.create_account();

        bank.deposit_cash(&account_number, 1400 as usize).unwrap();
        bank.withdraw_cash(&account_number, 400 as usize).unwrap();
        assert_eq!(bank.check_balance(&account_number), Some(1000 as usize));
    }

    #[test]
    fn should_give_error_when_account_has_insufficient_balance_for_debit() {
        let bank = Bank::new();
        let account_number = bank.create_account();

        bank.deposit_cash(&account_number, 1400 as usize).unwrap();
        assert!(bank.withdraw_cash(&account_number, 2000 as usize).is_err())
    }

    #[test]
    fn should_give_error_when_account_does_not_exists() {
        let bank = Bank::new();

        assert_eq!(bank.withdraw_cash("0123", 400 as usize), Err("Account not found".to_string()));
        assert_eq!(bank.deposit_cash("0123", 400 as usize), Err("Account not found".to_string()));
    }

    #[test]
    fn should_given_none_when_account_does_not_exists() {
        let bank = Bank::new();

        assert!(bank.check_balance("0123").is_none())
    }

    #[test]
    fn should_give_account_number_in_increasing_order_starting_from_1() {
        let bank = Bank::new();

        for i in 1..10 {
            assert_eq!(bank.generate_account_number(), i.to_string())
        }
    }

    #[test]
    fn should_give_transaction_history_of_an_account() {
        let bank = Bank::new();
        let account_number = bank.create_account();

        bank.deposit_cash(&account_number, 1400 as usize).unwrap();
        bank.withdraw_cash(&account_number, 400 as usize).unwrap();

        let transactions = bank.get_transactions(&account_number);
        assert_eq!(transactions, Some(vec![Transaction::credit_of_amount(1400 as usize), Transaction::debit_of_amount(400 as usize)]))
    }

    #[test]
    fn should_give_none_for_transaction_history_when_account_does_not_exists() {
        let bank = Bank::new();

        assert!(bank.get_transactions("12").is_none())
    }

    #[test]
    fn should_give_empty_vector_for_transaction_history_when_account_does_not_have_any_transaction() {
        let bank = Bank::new();
        let account_number = bank.create_account();
        assert_eq!(bank.get_transactions(&account_number), Some(vec![]))
    }
}