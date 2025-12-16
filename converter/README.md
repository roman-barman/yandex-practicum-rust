# converter

CLI tool to convert bank statement files between formats and/or pretty‑print them.

Supported formats:
- `mt940` — SWIFT Customer Statement Message (text)
- `camt053` — ISO 20022 BankToCustomerStatement (XML)

Build
```
cargo build -p converter
```

Usage
Show help:
```
cargo run -p converter -- --help
```

Convert MT940 → CAMT.053 (XML):
```
cargo run -p converter -- \
  -i ../file_examples/mt940/mt940.txt \
  -f mt940 \
  -o camt053 > out_camt053.xml
```

Convert CAMT.053 → MT940:
```
cargo run -p converter -- \
  -i ../file_examples/camt053/camt053.xml \
  -f camt053 \
  -o mt940 > out_mt940.txt
```

Pretty‑print without changing format (omit `-o` to use `Display` output):
```
cargo run -p converter -- -i ../file_examples/mt940/mt940.txt -f mt940
```

See also
- Workspace README with more details and examples: `../README.md`
- Library crate used by this tool: `../parser`
