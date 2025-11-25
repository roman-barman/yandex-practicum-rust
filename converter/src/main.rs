use clap::Parser;
use parser::Mt940CustomerStatementMessage;

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
                        println!("{}", statement);
                    }
                }
                Err(err) => match err.inner() {
                    Some(inner_err) => println!("Error: {}", inner_err),
                    None => println!("Error: {}", err),
                },
            }
        }
    }
}
