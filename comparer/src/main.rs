use crate::args::Format;
use clap::Parser;
use parser::{Camt053Message, Mt940CustomerStatementMessage, Transaction, TransactionProvider};
use std::collections::HashSet;

mod args;

fn main() {
    let args = args::Args::parse();
    let transactions1 = match get_transactions(args.file1_format, &args.file1) {
        Ok(transactions) => transactions,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    let transactions2 = match get_transactions(args.file2_format, &args.file2) {
        Ok(transactions) => transactions,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    let difference = transactions1.symmetric_difference(&transactions2);
    let mut has_difference = false;
    for transaction in difference {
        has_difference = true;
        println!("---");
        println!("{}", transaction)
    }

    if !has_difference {
        println!("No difference found");
    }
}

fn get_transactions(
    format: Format,
    file_path: &std::path::PathBuf,
) -> Result<HashSet<Transaction>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file: {}", file_path.display()));
    match format {
        Format::MT940 => {
            let mut transactions = HashSet::new();
            for statement in Mt940CustomerStatementMessage::read_from(file)? {
                transactions.extend(statement.get_transactions());
            }
            Ok(transactions)
        }
        Format::CAMT053 => Ok(Camt053Message::read_from(file)?.get_transactions()),
    }
}
