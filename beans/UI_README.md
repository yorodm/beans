# Beans UI - Tauri + Leptos Implementation

This directory contains the UI implementation for Beans using Tauri 2.0 and Leptos 0.7.

## Architecture

The UI is built using:
- **Tauri 2.0**: Cross-platform desktop app framework
- **Leptos 0.7**: Reactive UI framework (CSR mode)
- **Rust Backend**: Tauri commands that interface with beans-lib

## Project Structure

```
beans/
├── src/
│   ├── main.rs              # Tauri application entry point
│   ├── lib.rs               # Leptos app entry point
│   ├── commands.rs          # Tauri command handlers (backend)
│   ├── components/
│   │   ├── mod.rs
│   │   └── ribbon.rs        # Ribbon toolbar component
│   └── views/
│       ├── mod.rs
│       ├── overview.rs      # Overview with income/expense chart
│       ├── add_entry.rs     # Add new entry form
│       ├── edit_entry.rs    # Edit/delete entry view
│       └── export.rs        # Export ledger view
├── style/
│   └── main.css             # Application styles
├── tauri-conf/
│   └── tauri.conf.json      # Tauri configuration
├── build.rs                 # Build script
├── index.html               # HTML entry point
└── Cargo.toml               # Dependencies

```

## Features

### Ribbon Toolbar
- **Open/Create Ledger**: File dialog to open existing or create new .bean files
- **Overview**: Navigate to overview with income vs expenses chart
- **Add Entry**: Navigate to add new entry form
- **Edit Entry**: Navigate to edit/delete existing entries
- **Export**: Navigate to export functionality

### Views

#### 1. Overview
- Displays current date
- Shows bar graph comparing income vs expenses
  - Green bar for income
  - Red bar for expenses
  - Bars are labeled with amounts
- Filter by date range (start/end date)
- Filter by tags (comma-separated)
- Real-time chart updates when filters are applied

#### 2. Add New Entry
- Date picker (defaults to today)
- Entry type selector (Income/Expense)
- Name field (required)
- Currency selector (USD, EUR, GBP, JPY, CAD, AUD)
- Amount input (required, decimal)
- Description textarea (optional)
- Tags input (comma-separated, optional)
- Form validation with visual feedback

#### 3. Edit Entry
- Date filter to find entries by date
- Tag filter to find entries by tags
- List of entries matching filters
- Click to select an entry
- Edit form (same fields as Add Entry)
- Update button to save changes
- Delete button with confirmation dialog

#### 4. Export Ledger
- Choose export format (JSON or CSV)
- Optional filters:
  - Date range
  - Tags
  - Currency
  - Entry type (All/Income/Expense)
- File save dialog
- Formats supported by beans-lib

## Building and Running

### Prerequisites
- Rust toolchain (via rustup)
- Node.js and npm (for WASM tools)
- System dependencies for Tauri:
  - **Linux**: `webkit2gtk`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `libjavascriptcoregtk-4.0-dev`, `libsoup-3.0-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: WebView2 (usually pre-installed on Windows 11)

### Development Build

```bash
# Install wasm-pack if not already installed
cargo install wasm-pack

# Install trunk for serving
cargo install trunk

# Build and run in development mode
cargo tauri dev
```

### Production Build

```bash
# Build for production
cargo tauri build
```

The compiled application will be in `target/release/bundle/`.

## Tauri Commands

Backend commands exposed to the frontend:

- `open_ledger(path: String)`: Open an existing ledger file
- `create_ledger(path: String)`: Create a new ledger file
- `add_entry(entry: EntryData)`: Add a new entry to the ledger
- `update_entry(entry: EntryData)`: Update an existing entry
- `delete_entry(id: String)`: Delete an entry by ID
- `get_entries()`: Get all entries
- `get_entries_filtered(filter: FilterParams)`: Get filtered entries
- `get_report_data(filter: FilterParams)`: Get income vs expenses report
- `export_ledger(format: String, path: String, filter: Option<FilterParams>)`: Export ledger

## Styling

The application uses a custom CSS stylesheet (`style/main.css`) with:
- Modern, clean design
- Responsive layout
- Color-coded income (green) and expense (red)
- Dark ribbon toolbar
- Accessible form controls
- Mobile-friendly responsive design

## Key Components

### Ribbon Component
Located in `src/components/ribbon.rs`. Provides navigation and file operations.

### Overview View
Located in `src/views/overview.rs`. Custom SVG bar chart implementation showing income vs expenses.

### Add Entry View
Located in `src/views/add_entry.rs`. Form for creating new ledger entries.

### Edit Entry View
Located in `src/views/edit_entry.rs`. List and edit interface for existing entries.

### Export View
Located in `src/views/export.rs`. Export functionality with optional filters.

## Development Notes

### Leptos CSR Mode
The UI runs in Client-Side Rendering (CSR) mode, which is ideal for desktop apps. All UI logic runs in the browser component embedded in Tauri.

### State Management
- Tauri manages application state (current ledger) on the backend
- Leptos signals manage reactive UI state on the frontend
- Communication via Tauri commands (async)

### Error Handling
- All Tauri commands return `Result<T, String>`
- Errors are displayed using browser alert dialogs
- Frontend validates input before sending to backend

## Future Enhancements

Potential improvements:
- [ ] More sophisticated charting with leptos_chart or other libraries
- [ ] Multiple currency support in overview
- [ ] Trend analysis over time
- [ ] Budget tracking
- [ ] Recurring entries
- [ ] Data import functionality
- [ ] Custom themes
- [ ] Settings panel

## License

MIT License - see LICENSE file in project root.

