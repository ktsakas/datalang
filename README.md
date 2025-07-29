# DataLang

A Rust procedural macro for defining data structures using a custom dictionary/term syntax.

## Project Structure

```
datalang/
├── src/lib.rs                          # 🔧 Macro implementation
├── examples/basic_usage.rs             # 📝 Single-file comprehensive demo
├── tests/                              # 🧪 Integration tests (separate workspace)
│   ├── src/
│   │   ├── base.rs                     # Base dictionary definitions
│   │   ├── social_media.rs             # Social media extensions
│   │   ├── lib.rs                      # Module exports
│   │   └── main.rs                     # Main integration test
│   └── examples/
│       ├── base_demo.rs                # Base functionality demo
│       ├── social_media_demo.rs        # Social media demo
│       └── modular_demo.rs             # Cross-module usage demo
├── syntax.md                           # 📖 Syntax specification
└── Cargo.toml                          # Workspace root
```

## Usage

### Quick Start (Single File)
```bash
cargo run --example basic_usage
```

### Modular Usage (Integration Tests)
```bash
cd tests
cargo run                              # Main integration test
cargo test --example base_demo         # Run base functionality tests
cargo test --example social_media_demo # Run social media tests  
cargo test --example modular_demo      # Run cross-module tests
cargo test                             # Run all tests
```

## Features

- **Dictionary definitions**: `dictionary Base`
- **Simple terms**: `term Name {}`
- **Composite terms**: `term User has { +Name +LastName +BirthDate }`
- **Selective inclusion**: `SocialMediaUser { +Name +BirthDate +Handle }`
- **Import system**: `import Base`
- **Field exclusion**: Omit unwanted fields

## Why Two Approaches?

1. **`examples/basic_usage.rs`**: Single-file demo showing all features
2. **`tests/`**: Separate workspace demonstrating modular usage across files

This is necessary because procedural macro crates cannot use their own macros directly.