# comparer

CLI tool to compare transactions between two bank statement files. Inputs may be in the same or different formats.

Supported formats:
- `mt940` — SWIFT Customer Statement Message (text)
- `camt053` — ISO 20022 BankToCustomerStatement (XML)

Build
```
cargo build -p comparer
```

Usage
Show help:
```
cargo run -p comparer -- --help
```

Compare transactions across two files:
```
cargo run -p comparer -- \
  --file1 ../file_examples/mt940/mt940.txt --file1-format mt940 \
  --file2 ../file_examples/camt053/camt053.xml --file2-format camt053
```

The tool prints symmetric differences; if none found, it reports "No difference found".

See also
- Workspace README with more details and examples: `../README.md`
- Library crate used by this tool: `../parser`
