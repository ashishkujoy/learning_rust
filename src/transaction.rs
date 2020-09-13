#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Transaction {
    pub amount: usize,
    pub transaction_type: TransactionType,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TransactionType {
    CashDeposit,
    CashWithdrawal,
}

impl Transaction {
    pub fn credit_of_amount(amount: usize) -> Self {
        Transaction { amount, transaction_type: TransactionType::CashDeposit }
    }

    pub fn debit_of_amount(amount: usize) -> Self {
        Transaction { amount, transaction_type: TransactionType::CashWithdrawal }
    }
}