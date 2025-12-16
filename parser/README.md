# parser

Library crate with domain types and readers/writers for bank statement formats.

Features
- `Mt940CustomerStatementMessage` — parse/print MT940 (SWIFT Customer Statement Message)
- `Camt053Message` — parse/print CAMT.053 (ISO 20022 BankToCustomerStatement)
- `MessageWriter` trait — stream output helper

Add to your `Cargo.toml`:
```
[dependencies]
parser = { path = "..//parser" }
```

Example: read CAMT.053 and write it back to stdout
```rust
use parser::{Camt053Message, MessageWriter};
use std::fs::File;
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../file_examples/camt053/camt053.xml")?;
    let message = Camt053Message::read_from(file)?;
    let mut out = stdout();
    message.write_to(&mut out)?;
    Ok(())
}
```

See also
- Workspace README with more details and examples: `../README.md`
- CLI tools using this library: `../converter` and `../comparer`

Examples folder
- You can run ready-to-use examples via Cargo:
  - CAMT.053: `cargo run -p parser --example camt053_read_write`
  - MT940: `cargo run -p parser --example mt940_read_write`
  These examples read files from `file_examples/` (workspace root) and print the parsed content back to stdout.
  For MT940, you may pass a specific sample file path, e.g.:
  `cargo run -p parser --example mt940_read_write "file_examples/mt940/MT_940 aiophotoz.mt940"`
