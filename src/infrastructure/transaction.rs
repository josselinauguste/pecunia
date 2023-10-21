use std::error::Error;

use crate::domain::transaction::{Transaction, TransactionRepository};

pub struct LogTransactionRepository;

impl TransactionRepository for LogTransactionRepository {
    fn append(&self, transactions: Vec<&Transaction>) -> Result<(), Box<dyn Error>> {
        transactions
            .iter()
            .for_each(|transaction| println!("{:?}", transaction));
        Ok(())
    }
}
