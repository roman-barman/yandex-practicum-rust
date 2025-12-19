use crate::args::Format;
use clap::Parser;
use parser::{Camt053Message, Mt940CustomerStatementMessage, Transaction, TransactionProvider};

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
    let difference_with_file2 = get_difference(&transactions1, &transactions2);
    let difference_with_file1 = get_difference(&transactions2, &transactions1);
    let mut has_difference = false;
    for transaction in difference_with_file2 {
        has_difference = true;
        println!("---");
        println!("Not existing in file 2");
        println!("{}", transaction)
    }
    for transaction in difference_with_file1 {
        has_difference = true;
        println!("---");
        println!("Not existing in file 1");
        println!("{}", transaction)
    }

    if !has_difference {
        println!("No difference found");
    }
}

fn get_difference<'a>(
    transactions1: &'a [Transaction],
    transactions2: &'a [Transaction],
) -> Vec<&'a Transaction> {
    transactions1
        .iter()
        .filter(|t1| transactions2.iter().all(|t2| !t1.eq(&t2)))
        .collect::<Vec<&Transaction>>()
}

fn get_transactions(
    format: Format,
    file_path: &std::path::PathBuf,
) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file: {}", file_path.display()));
    match format {
        Format::MT940 => {
            let mut transactions = Vec::new();
            for statement in Mt940CustomerStatementMessage::read_from(file)? {
                transactions.extend(statement.get_transactions());
            }
            Ok(transactions)
        }
        Format::CAMT053 => Ok(Camt053Message::read_from(file)?.get_transactions()),
    }
}
