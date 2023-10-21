use bigdecimal::BigDecimal;
use std::error::Error;
use std::fs::File;
use std::io;

use crate::application::cutoff::{CutoffDecoder, CutoffStorage};
use crate::domain::transaction::Transaction;

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct N26CsvCutoff {
    date: chrono::NaiveDate,
    payee: String,
    account_number: String,
    transaction_type: String,
    payment_reference: String,
    amount_eur: BigDecimal,
    amount_foreign_currency: Option<BigDecimal>,
    type_foreign_currency: String,
    exchange_rate: String,
}

pub struct N26CsvCutoffDecoder {}

impl CutoffDecoder for N26CsvCutoffDecoder {
    fn decode(&self, record: &csv::StringRecord) -> Result<Transaction, Box<dyn Error>> {
        record
            .deserialize(None)
            .map_err(|e| e.into())
            .map(|c: N26CsvCutoff| Transaction {
                date: c.date,
                payee: c.payee,
                reference: c.payment_reference,
                amount: c.amount_eur,
                source: "n26".to_string(),
            })
    }
}

pub struct N26CsvCutoffFile {
    pub filename: String,
}

impl CutoffStorage<N26CsvCutoffDecoder> for N26CsvCutoffFile {
    fn read(&self) -> io::Result<File> {
        File::open(&self.filename)
    }

    fn decoder(&self) -> N26CsvCutoffDecoder {
        N26CsvCutoffDecoder {}
    }
}
