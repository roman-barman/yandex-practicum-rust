use parser::{MessageWriter, Mt940CustomerStatementMessage};
use std::fs::File;
use std::io::{self, Write};

// Read an MT940 file and write parsed messages back to stdout
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example file bundled in this workspace (can be overridden by first CLI arg)
    let path = "file_examples/mt940/mt 940 gs.mt940";

    let file = File::open(path)?;
    let messages = Mt940CustomerStatementMessage::read_from(file)?;

    let mut out = io::stdout();
    for message in messages {
        message.write_to(&mut out)?;
        writeln!(&mut out)?;
    }
    Ok(())
}
