# Beans 🫘

A multi-platform ledger application built with Rust, designed for tracking income and expenses with powerful reporting capabilities.

## Features

- ✅ **Ledger Management**: Create and manage ledger files with `.bean` extension
- ✅ **Entry Tracking**: Add, update, and delete ledger entries
- ✅ **Rich Entry Data**: Date, name, currency, amount, description, tags, and income/expense type
- ✅ **Filtering & Querying**: Filter entries by date range, tags, currency, and type  
- ✅ **Currency Conversion**: Convert between currencies using live exchange rates
- ✅ **Reports & Analytics**: Generate income vs expense reports with time-series data
- ✅ **Multi-Platform**: Works on Windows, macOS, and Linux
- 🚧 **UI (Phase 2)**: Graphical interface using Ribir 0.2.x (coming soon)

## Project Structure

```
beans/
├── Cargo.toml          # Workspace configuration
├── beans-lib/          # Core library (business logic)
│   ├── src/
│   │   ├── error.rs    # ✅ Error types
│   │   ├── lib.rs      # ✅ Public API
│   │   ├── models/     # ✅ Domain models
│   │   │   ├── currency.rs  # ✅ Currency type with rusty-money
│   │   │   ├── entry.rs     # ✅ LedgerEntry with builder
│   │   │   └── tag.rs       # ✅ Tag with validation
│   │   ├── database/   # ✅ SQLite persistence
│   │   ├── ledger/     # ✅ Ledger manager
│   │   ├── currency/   # ✅ Currency conversion
│   │   └── reporting/  # 🚧 Analytics & reporting
│   ├── tests/          # ✅ Integration tests for models, ledger, and currency
│   └── examples/       # ✅ Comprehensive examples for all features
└── beans/              # Binary crate (UI application)
    └── src/
        └── main.rs     # ✅ Placeholder (UI in Phase 2)
```

## Development Status

### Phase 1: Core Library (In Progress)

**Completed:**
- ✅ Project structure with Cargo workspace
- ✅ Error handling with thiserror
- ✅ Dependencies configured (rusqlite, serde, chrono, reqwest, etc.)
- ✅ Basic compilation verified
- ✅ Currency implementation with rusty-money (type-safe, with proper error handling)
- ✅ Tag model with validation and normalization
- ✅ LedgerEntry model with builder pattern
- ✅ Database layer with SQLite
- ✅ Ledger manager service with file handling
- ✅ Currency conversion with external API integration

**In Progress:**
- 🚧 Reporting and analytics
- 🚧 Additional unit and integration tests

### Phase 2: UI Implementation (Planned)

- [ ] Ribir 0.2.x integration
- [ ] Entry management UI
- [ ] Filtering and search UI
- [ ] Graph visualization for income/expenses
- [ ] Currency conversion UI
- [ ] Settings and preferences

## Development Environment

### Using Nix Flake (Recommended)

This project includes a Nix flake for reproducible development environments across all platforms.

**Prerequisites:**
- [Nix package manager](https://nixos.org/download.html) with flakes enabled
- Optional: [direnv](https://direnv.net/) for automatic environment activation

**Setup:**

```bash
# Enter development shell directly
nix develop

# OR with direnv (automatic activation)
direnv allow
```

The development environment provides:
- Rust toolchain with rustfmt, clippy, and rust-analyzer
- All required system dependencies (SQLite, OpenSSL)
- Development tools (cargo-watch, cargo-audit, sqlitebrowser)
- Cross-platform compatibility (Linux, macOS)

### Using Cargo Only

If you prefer not to use Nix, you'll need to install the following dependencies manually:
- Rust toolchain (via [rustup](https://rustup.rs/))
- SQLite development libraries
- OpenSSL development libraries

## Building

```bash
# Build the library
cargo build --lib

# Build the binary (placeholder for now)
cargo build --bin beans

# Run tests (once implemented)
cargo test

# Build documentation
cargo doc --open

# Development with auto-reload
cargo watch -x run
```

## Architecture

### Core Library (beans-lib)

**Models Module** (`models/`)
- `Currency`: ISO 4217 currency codes with validation
- `Tag`: Entry categorization with normalization
- `LedgerEntry`: Complete entry with all fields
- `EntryType`: Income vs Expense enum

**Database Module** (`database/`)
- SQLite schema with migrations
- Repository pattern for CRUD operations
- Entry filtering with complex queries
- Tag management with many-to-many relationships

**Ledger Module** (`ledger/`)
- `LedgerManager`: High-level ledger operations
- File management (.bean extension enforcement)
- Business logic and validation

**Currency Module** (`currency/`)
- `CurrencyConverter`: Exchange rate fetching
- `ExchangeRateCache`: Caching with TTL
- Integration with https://github.com/fawazahmed0/exchange-api

**Reporting Module** (`reporting/`)
- `ReportGenerator`: Analytics and aggregations  
- Time-series data generation
- Income vs expense calculations
- Multi-currency support with normalization

## Dependencies

- **rusqlite**: SQLite database
- **serde**: Serialization/deserialization
- **chrono**: Date and time handling
- **thiserror**: Error handling
- **reqwest + tokio**: Async HTTP client
- **uuid**: Unique identifiers
- **rust_decimal**: Precise decimal math for currency

## License

MIT License - see LICENSE file for details

## Contributing

This project is currently in early development. Phase 1 (core library) must be completed and reviewed before Phase 2 (UI) begins.

## AI Assistance Disclaimer

This project was developed with the assistance of an AI coding agent (Codegen). The AI helped with:

- Initial project architecture and structure
- Implementation of placeholder modules
- Documentation and examples
- Best practices for Rust development

While AI provided significant assistance, all code has been reviewed and validated to ensure it meets quality standards and follows Rust best practices.

## Roadmap

1. ✅ Initialize project structure
2. ✅ Implement domain models (Currency, Tag, LedgerEntry)
3. ✅ Implement database layer with SQLite
4. ✅ Implement ledger manager service
5. ✅ Implement currency conversion with external API
6. 🚧 Implement reporting and analytics
7. 🚧 Add comprehensive tests (unit + integration)
8. ✅ Write documentation and examples
9. [ ] Review and refine Phase 1
10. [ ] Implement UI with Ribir 0.2.x
11. [ ] Add graph visualization
12. [ ] Package and distribute

---

**Status**: Phase 1 - Core Library Development (In Progress, ~70% Complete)
