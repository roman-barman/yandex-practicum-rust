# Bank Statement Parser

Rust workspace with small tools and a library for working with bank statement formats:
- MT940 (SWIFT Customer Statement Message)
- CAMT.053 (ISO 20022 BankToCustomerStatement)

These are the actual interchange formats used between banks and corporate clients. The library provides typed domain models with parsing and printing for both, plus CLI tools to convert between formats and reconcile transactions across files.

**Highlights:**
- Hand-written parser for the SWIFT MT940 tag-based text format
- CAMT.053 XML parsing into shared domain types
- Format-agnostic transaction comparison (reconciliation across MT940/CAMT.053 in any combination)
- Library-first design: CLIs are thin wrappers over the reusable `parser` crate

**Stack:** Rust · Clap

## Repository Structure

- parser — reusable library with domain types and readers/writers for MT940 and CAMT.053
- converter — CLI to convert statements between MT940 and CAMT.053 or pretty‑print them
- comparer — CLI to compare transactions between two statement files (same or different formats)

## Requirements

Rust and Cargo installed.

## Build

```
cargo build --workspace
```

## Repository structure

- converter/ — CLI tool (uses the `parser` crate)
- comparer/ — CLI tool (uses the `parser` crate)
- parser/ — library crate with:
  - `Mt940CustomerStatementMessage` — MT940 parsing/printing
  - `Camt053Message` — CAMT.053 parsing/printing
  - `MessageWriter` trait — stream output helper
- file_examples/ — sample input files for both formats

## Usage — converter

Show help:
```
cargo run -p converter -- --help
```

Convert MT940 → CAMT.053 (XML):
```
cargo run -p converter -- \
  -i file_examples/mt940/mt940.txt \
  -f mt940 \
  -o camt053 > out_camt053.xml
```

Convert CAMT.053 → MT940:
```
cargo run -p converter -- \
  -i file_examples/camt053/camt053.xml \
  -f camt053 \
  -o mt940 > out_mt940.txt
```

Pretty‑print without changing format (omit `-o` to use Display output):
```
cargo run -p converter -- -i file_examples/mt940/mt940.txt -f mt940
```

## Usage — comparer

Compare transactions across two files (any combination of formats):
```
cargo run -p comparer -- \
  --file1 file_examples/mt940/mt940.txt --file1-format mt940 \
  --file2 file_examples/camt053/camt053.xml --file2-format camt053
```
The tool prints symmetric differences; if none found, it reports "No difference found".

## Library usage (parser)

Example: read CAMT.053 and print back to stdout via `MessageWriter`:
```rust
use parser::{Camt053Message, MessageWriter};
use std::fs::File;
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("file_examples/camt053/camt053.xml")?;
    let message = Camt053Message::read_from(file)?;
    let mut out = stdout();
    message.write_to(&mut out)?;
    Ok(())
}
```

## Run tests

```
cargo test --workspace
```

## Notes
- Sample files are located under `file_examples/mt940` and `file_examples/camt053`.
- All CLIs use Clap, so `--help` shows the available options and formats.
