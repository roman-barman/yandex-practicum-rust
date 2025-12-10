use clap::Parser;
use parser::{Camt053Message, Mt940CustomerStatementMessage};
use std::io::Write;

mod args;

fn main() {
    let args = args::Args::parse();
    match args.input_format {
        args::InputFormat::MT940 => {
            let file = std::fs::File::open(&args.input).expect("Unable to read file");
            let result = Mt940CustomerStatementMessage::read_from(file);
            match result {
                Ok(statements) => {
                    for statement in statements {
                        match args.output_format {
                            None => println!("{}", statement),
                            Some(args::OutputFormat::MT940) => {
                                let mut stdout = std::io::stdout();
                                statement
                                    .write_to(&mut stdout)
                                    .expect("Unable to write to stdout");
                                stdout.flush().expect("Unable to flush stdout");
                            }
                        }
                    }
                }
                Err(err) => eprintln!("{}", err),
            }
        }
        args::InputFormat::CAMT053 => {
            let file = std::fs::File::open(&args.input).expect("Unable to read file");
            let result = Camt053Message::read_from(file);
            match result {
                Ok(message) => println!("{}", message),
                Err(err) => eprintln!("{}", err),
            }
        }
    }
}
