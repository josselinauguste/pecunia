use bigdecimal::BigDecimal;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Transaction {
    pub date: NaiveDate,
    pub payee: String,
    pub reference: String,
    pub amount: BigDecimal,
    pub source: String,
}

pub trait TransactionRepository {
    fn append(&self, transactions: Vec<&Transaction>) -> Result<(), Box<dyn std::error::Error>>;
}
