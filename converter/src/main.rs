use clap::Parser;
use parser::{Camt053Message, Mt940CustomerStatementMessage};
use std::io::Write;

mod args;

fn main() {
    let args = args::Args::parse();
    let mut stdout = std::io::stdout();
    match args.input_format {
        args::Format::MT940 => {
            let file = std::fs::File::open(&args.input).expect("Unable to read file");
            let result = Mt940CustomerStatementMessage::read_from(file);
            match result {
                Ok(statements) => {
                    for statement in statements {
                        match args.output_format {
                            None => println!("{}", statement),
                            Some(args::Format::MT940) => {
                                statement
                                    .write_to(&mut stdout)
                                    .expect("Unable to write to stdout");
                            }
                            Some(args::Format::CAMT053) => {
                                let camt053_message = Camt053Message::from(&statement);
                                camt053_message
                                    .write_to(&mut stdout)
                                    .expect("Unable to write to stdout");
                            }
                        }
                    }
                }
                Err(err) => eprintln!("{}", err),
            }
        }
        args::Format::CAMT053 => {
            let file = std::fs::File::open(&args.input).expect("Unable to read file");
            let result = Camt053Message::read_from(file);
            match result {
                Ok(message) => match args.output_format {
                    None => println!("{}", message),
                    Some(args::Format::MT940) => {
                        for statement in message.get_statements() {
                            let mt940 = Mt940CustomerStatementMessage::try_from(statement);
                            match mt940 {
                                Ok(mt940) => {
                                    mt940
                                        .write_to(&mut stdout)
                                        .expect("Unable to write to stdout");
                                }
                                Err(err) => {
                                    eprintln!("{}", err);
                                    break;
                                }
                            }
                        }
                    }
                    Some(args::Format::CAMT053) => {
                        message
                            .write_to(&mut stdout)
                            .expect("Unable to write to stdout");
                    }
                },
                Err(err) => eprintln!("{}", err),
            }
        }
    }
    stdout.flush().expect("Unable to flush stdout");
}
