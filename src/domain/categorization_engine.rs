use crate::domain::transaction::Transaction;

mod rules;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Category {
    LunchExtra,
    Vinyls,
    Books,
}

pub fn categorize(transaction: &Transaction) -> Option<Category> {
    rules::rules()
        .iter()
        .find(|rule| transaction.payee.starts_with(&rule.pattern))
        .map(|rule| rule.category)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    #[test]
    fn categorize_transaction() {
        let transaction = build_transaction("BISTRO MONTES".to_string());
        let expected = Category::LunchExtra;

        let result = categorize(&transaction);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn not_categorize_unknown_transaction() {
        let transaction = build_transaction("FREAKY STUFF".to_string());

        let result = categorize(&transaction);

        assert!(result.is_none());
    }

    fn build_transaction(payee: String) -> Transaction {
        let transaction = Transaction {
            date: chrono::NaiveDate::parse_from_str("2019-01-01", "%Y-%m-%d").unwrap(),
            payee,
            reference: "-".to_string(),
            amount: BigDecimal::from(10),
            source: "n26".to_string(),
        };
        transaction
    }
}
