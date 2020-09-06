use std::cell::{Cell, RefCell, Ref};
use crate::transaction::{Transaction, TransactionType};

struct Account {
    account_number: String,
    balance: Cell<usize>,
    transactions: RefCell<Vec<Transaction>>,
}

unsafe impl Sync for Account {}

impl Account {
    fn new(account_number: String) -> Self {
        Account {
            account_number,
            balance: Cell::new(0),
            transactions: RefCell::new(vec![]),
        }
    }

    fn perform_transaction(&self, transaction: Transaction) -> Result<(), String> {
        let current_balance = self.balance.get();
        let mut transactions = self.transactions.borrow_mut();
        match transaction.transaction_type {
            TransactionType::CashDeposit => {
                self.balance.set(current_balance + transaction.amount);
                transactions.push(transaction);
                Ok(())
            }
            TransactionType::CashWithdrawal => {
                if current_balance >= transaction.amount {
                    self.balance.set(current_balance - transaction.amount);
                    transactions.push(transaction);
                    Ok(())
                } else {
                    Err("Insufficient balance".to_string())
                }
            }
        }
    }

    fn transactions_history(&self) -> Ref<'_, Vec<Transaction>> {
        self.transactions.borrow()
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
        let account = Account::new("abcd1234".to_string());
        account.perform_transaction(Transaction::credit_of_amount(1000.45 as usize)).unwrap();
        assert_eq!(account.get_balance(), 1000.45 as usize)
    }

    #[test]
    fn should_debit_from_account() {
        let account = Account::new("abcd1234".to_string());
        account.perform_transaction(Transaction::credit_of_amount(1000.45 as usize)).unwrap();
        account.perform_transaction(Transaction::debit_of_amount(100 as usize)).unwrap();
        assert_eq!(account.get_balance(), 900.45 as usize)
    }

    #[test]
    fn should_give_error_for_debit_on_insufficient_balance() {
        let account = Account::new("abcd1234".to_string());
        account.perform_transaction(Transaction::credit_of_amount(1000.45 as usize)).unwrap();
        let error = account.perform_transaction(Transaction::debit_of_amount(2000 as usize));

        assert_eq!(error, Err("Insufficient balance".to_string()));
    }

    #[test]
    fn should_give_transaction_history() {
        let account = Account::new("abcd1234".to_string());
        account.perform_transaction(Transaction::credit_of_amount(1000.45 as usize)).unwrap();
        account.perform_transaction(Transaction::debit_of_amount(200 as usize)).unwrap();
        let expected_history = vec![
            Transaction::credit_of_amount(1000.45 as usize),
            Transaction::debit_of_amount(200 as usize),
        ];

        assert_eq!(account.transactions_history().get(0), expected_history.get(0));
        assert_eq!(account.transactions_history().get(1), expected_history.get(1));
        assert_eq!(account.transactions_history().get(2), None);
    }
}