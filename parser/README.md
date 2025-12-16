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
