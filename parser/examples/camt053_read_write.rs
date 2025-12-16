use parser::{Camt053Message, MessageWriter};
use std::fs::File;
use std::io::{self, Write};

// Read a CAMT.053 XML file and write it back to stdout
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example file bundled in this workspace
    let path = "file_examples/camt053/camt 053 danske bank.xml";

    let file = File::open(path)?;
    let message = Camt053Message::read_from(file)?;

    let mut out = io::stdout();
    message.write_to(&mut out)?;
    // newline for readability when piping output
    writeln!(&mut out)?;
    Ok(())
}
