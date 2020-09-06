use std::cell::Cell;

struct Account {
    account_number: String,
    balance: Cell<usize>,
}

unsafe impl Sync for Account {}

struct Transaction {
    amount: usize,
    transaction_type: TransactionType,
}

enum TransactionType {
    CashDeposit,
    CashWithdrawal,
}

impl Transaction {
    fn credit_of_amount(amount: usize) -> Self {
        Transaction { amount, transaction_type: TransactionType::CashDeposit }
    }

    fn debit_of_amount(amount: usize) -> Self {
        Transaction { amount, transaction_type: TransactionType::CashWithdrawal }
    }
}

impl Account {
    fn new(account_number: String) -> Self {
        Account { account_number, balance: Cell::new(0) }
    }

    pub fn credit(&self, transaction: Transaction) {
        let current_balance = self.balance.get();
        self.balance.set(current_balance + transaction.amount)
    }

    pub fn debit(&self, transaction: Transaction) -> Result<(), String> {
        let current_balance = self.balance.get();
        if current_balance >= transaction.amount {
            self.balance.set(current_balance - transaction.amount);
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
        let account = Account::new("abcd1234".to_string());
        account.credit(Transaction::credit_of_amount(1000.45 as usize));
        assert_eq!(account.get_balance(), 1000.45 as usize)
    }

    #[test]
    fn should_debit_from_account() {
        let account = Account::new("abcd1234".to_string());
        account.credit(Transaction::credit_of_amount(1000.45 as usize));
        account.debit(Transaction::debit_of_amount(100 as usize)).unwrap();
        assert_eq!(account.get_balance(), 900.45 as usize)
    }

    #[test]
    fn should_give_error_for_debit_on_insufficient_balance() {
        let account = Account::new("abcd1234".to_string());
        account.credit(Transaction::credit_of_amount(1000.45 as usize));
        let error = account.debit(Transaction::debit_of_amount(2000 as usize));

        assert_eq!(error, Err("Insufficient balance".to_string()));
    }
}