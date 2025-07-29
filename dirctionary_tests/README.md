# DataLang Test Cases

This directory contains test cases for the DataLang syntax. These `.txt` files test various language features and are validated at build time to ensure the parser works correctly.

> **Why separate package?** Procedural macro crates cannot use their own macros during compilation, so tests must be in a separate package.

## Directory Structure

```
dirctionary_tests/
├── text_definitions/           # 📝 DataLang test cases
│   ├── base.txt               # Tests basic dictionary/term syntax
│   └── social_media.txt       # Tests imports and namespaces
└── macro_definitions/         # 🤖 Auto-generated test files
    ├── base.rs               # Generated from base.txt
    └── social_media.rs       # Generated from social_media.txt
```

## Test Cases

**`base.txt`** - Tests basic language features:
```datalang
dictionary Base
term Name {}
term User has { +Name }
```

**`social_media.txt`** - Tests imports and namespaces:
```datalang
import Base
term Handle {}
SocialMediaUser { +Base::Name +Handle }
```

## Build Process

Running `cargo build` validates test files and generates corresponding `.rs` files:
```bash
cargo build
# Output: DataLang: ✓ base.txt ✓ social_media.txt
```

## Adding Test Cases

1. Create a `.txt` file in `text_definitions/`
2. Add DataLang syntax to test specific features
3. Run `cargo build` to validate and generate test files

## Syntax Quick Reference

- **Dictionary**: `dictionary MyDict`
- **Import**: `import Base`
- **Simple term**: `term Name {}`
- **Composite term**: `term User has { +Name +LastName }`
- **Struct**: `MyStruct { +LocalField +OtherDict::RemoteField }`

For complete syntax specification, see `../syntax.md`.