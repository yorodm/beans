# Beans UI Setup Guide

## Quick Start

### System Requirements

**All Platforms:**
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Node.js 16+ (for WASM tooling)

**Platform-Specific:**

#### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Windows
- Install [Microsoft Edge WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### Development Tools

1. Install Tauri CLI:
```bash
cargo install tauri-cli --version "^2.0.0"
```

2. Install WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Install trunk (for building Leptos frontend):
```bash
cargo install trunk
```

### Building

#### Development Mode
```bash
cd beans
cargo tauri dev
```

This will:
1. Build the Rust backend
2. Build the Leptos frontend to WASM
3. Launch the application in development mode with hot reload

#### Production Build
```bash
cd beans
cargo tauri build
```

Output will be in `target/release/bundle/`.

### Project Structure

```
beans/
├── src/
│   ├── main.rs           # Tauri backend entry point
│   ├── lib.rs            # Leptos frontend entry point  
│   ├── commands.rs       # Tauri command handlers
│   ├── components/       # Reusable UI components
│   │   └── ribbon.rs     # Ribbon toolbar
│   └── views/            # Page views
│       ├── overview.rs
│       ├── add_entry.rs
│       ├── edit_entry.rs
│       └── export.rs
├── style/
│   └── main.css          # Application styles
├── tauri-conf/
│   └── tauri.conf.json   # Tauri configuration
├── index.html            # HTML entry point
├── build.rs              # Build script
└── Cargo.toml            # Dependencies
```

## Development Workflow

### 1. Start Development Server
```bash
cargo tauri dev
```

### 2. Making Changes

**Frontend Changes (Leptos):**
- Edit files in `src/lib.rs`, `src/components/`, or `src/views/`
- Changes will hot-reload automatically

**Backend Changes (Tauri commands):**
- Edit `src/commands.rs` or `src/main.rs`
- Application will rebuild automatically

**Styling Changes:**
- Edit `style/main.css`
- Refresh the application to see changes

### 3. Testing

```bash
# Test the library
cd ../beans-lib
cargo test

# Check for compilation errors
cd ../beans
cargo check
```

### 4. Building for Release

```bash
cargo tauri build --release
```

## Troubleshooting

### Build Errors

**"Failed to resolve package":**
- Run `cargo update` to update dependencies
- Check your Rust version: `rustc --version` (should be 1.70+)

**"WebView2 not found" (Windows):**
- Install [Microsoft Edge WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

**"webkit2gtk not found" (Linux):**
- Install system dependencies (see Linux section above)

### Runtime Errors

**"No ledger opened":**
- Use the "Open/Create" button in the ribbon to open or create a ledger first

**"Failed to open ledger":**
- Ensure the .bean file exists and is readable
- Check file permissions

**"Invalid date format":**
- Dates must be in YYYY-MM-DD format
- Use the date picker in the UI to avoid errors

## Configuration

### Tauri Configuration

Edit `tauri-conf/tauri.conf.json` to configure:
- Window size and properties
- Bundle settings
- Permissions
- Build options

### Styling

Edit `style/main.css` to customize:
- Colors (CSS variables at the top)
- Layout and spacing
- Font styles
- Responsive breakpoints

## Using Nix (Recommended)

If you're using the Nix development environment (recommended for reproducibility):

```bash
# Enter the development shell
nix develop

# Or with direnv
direnv allow

# Then run Tauri as normal
cargo tauri dev
```

The Nix environment provides all necessary dependencies including:
- Rust toolchain
- System libraries
- Development tools

## Next Steps

1. Read the [UI_README.md](./UI_README.md) for architecture details
2. Check out the [IMPLEMENTATION_GUIDE.md](../IMPLEMENTATION_GUIDE.md) for design decisions
3. Explore the beans-lib API in [../beans-lib/src/](../beans-lib/src/)

## Additional Resources

- [Tauri Documentation](https://tauri.app/v2/guides/)
- [Leptos Documentation](https://leptos.dev/)
- [beans-lib Documentation](../beans-lib/README.md)

