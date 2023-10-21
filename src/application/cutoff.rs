use std::error::Error;
use std::fs::File;
use std::io;

use crate::domain::transaction::{Transaction, TransactionRepository};

pub trait CutoffDecoder {
    fn decode(&self, record: csv::StringRecord) -> Result<Transaction, Box<dyn std::error::Error>>;
}

pub trait CutoffStorage<C>
where
    C: CutoffDecoder,
{
    fn read(&self) -> io::Result<File>;
    fn decoder(&self) -> C;
}

type ImportResult = Vec<Result<Transaction, Box<dyn Error>>>;

pub fn import<R, S, C>(
    transaction_repository: R,
    storage: S,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: TransactionRepository,
    S: CutoffStorage<C>,
    C: CutoffDecoder,
{
    let (ok, err): (ImportResult, ImportResult) = storage
        .read()
        .map(csv::Reader::from_reader)?
        .records()
        .map(|record| {
            record
                .map_err(|e| e.into())
                .and_then(|r| storage.decoder().decode(r))
        })
        .partition(Result::is_ok);
    if !err.is_empty() {
        return Err(format!("{} transactions failed to parse", err.len()).into());
    }
    transaction_repository.append(ok.iter().filter_map(|t| t.as_ref().ok()).collect())
}
