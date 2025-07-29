# DataLang

A Rust procedural macro for defining data structures using a custom dictionary/term syntax.

> **Note**: The `dirctionary_tests/` directory contains test cases for the DataLang syntax and must be a separate package because procedural macro crates cannot use their own macros directly during compilation.

## Project Structure

```
datalang/
├── src/
│   ├── lib.rs                              # 🔧 Procedural macro implementations
│   └── types.rs                            # 📊 Core types and parsing logic
├── build.rs                                # 🏗️ Build-time text validation
├── dirctionary_tests/                      # 🧪 Test cases (separate package)
│   ├── text_definitions/                   # 📝 DataLang test definitions  
│   │   ├── base.txt                        # Base dictionary tests
│   │   └── social_media.txt                # Social media extension tests
│   └── macro_definitions/                  # 🤖 Auto-generated test files
│       ├── base.rs                         # Generated from base.txt
│       └── social_media.rs                 # Generated from social_media.txt
├── syntax.md                               # 📖 Syntax specification
└── Cargo.toml                              # Main crate configuration
```

## Features

- **Dictionary/term syntax**: `dictionary Base`, `term User has { +Name +LastName }`
- **Build-time validation**: Validates `.txt` test files and generates corresponding `.rs` files  
- **Namespace support**: Reference fields across dictionaries with `Base::Name`
- **Code generation**: Creates Rust structs with `Debug`, `Clone`, and `new()` methods

## Usage

```rust
use datalang::datalang;

datalang! {
    dictionary Base
    
    term Name {}
    term User has { +Name }
}

fn main() {
    let user = User::new();
    println!("{:?}", user); // User { name: "" }
}
```

## Testing

The `dirctionary_tests/` directory contains language test cases. See the **[Test Guide](dirctionary_tests/README.md)** for details on the test structure and syntax validation.

## Generated Code

```rust  
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
}

impl User {
    pub fn new() -> Self {
        Self { name: String::new() }
    }
}
```

## Architecture

- **`types.rs`**: Core types and parsing logic shared between build script and macro
- **`lib.rs`**: Procedural macro implementation using syn parsing  
- **`build.rs`**: Validates test `.txt` files and generates `.rs` files for testing

## Development

- **Syntax specification**: See `syntax.md`
- **Test cases**: See `dirctionary_tests/` directory and its [README](dirctionary_tests/README.md)
- **Adding features**: Ensure compatibility with both string parsing (build.rs) and syn parsing (lib.rs)