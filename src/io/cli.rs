use std::error::Error;

use crate::application::cutoff::import;
use crate::infrastructure::cutoff::N26CsvCutoffFile;
use crate::infrastructure::transaction::LogTransactionRepository;

pub fn handle_command(args: &[String]) -> Result<(), Box<dyn Error>> {
    let command = &args[1];
    match command.as_str() {
        "import" => {
            let filename = &args[2];
            let cutoff_file = N26CsvCutoffFile {
                filename: filename.to_string(),
            };
            import(&LogTransactionRepository, &cutoff_file)
        }
        _ => Err(format!("Unknown command: {}", command).into()),
    }
}
