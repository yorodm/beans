# Beans UI

This is the graphical user interface for the Beans ledger application, built with [Dioxus](https://dioxuslabs.com/).

## Features

- **Ribbon-style toolbar**: Easy access to all main functions
- **Ledger management**: Open existing or create new `.bean` ledger files
- **Overview dashboard**: Visual income vs expenses with bar chart
- **Entry management**: Add and edit ledger entries
- **Advanced filtering**: Filter by date range and tags
- **Export functionality**: Export to JSON or CSV formats

## Views

### 1. Ledger Selection
- Open an existing `.bean` file
- Create a new ledger file
- Automatic `.bean` extension handling

### 2. Overview
- Current date display
- Interactive bar chart showing income (green) vs expenses (red)
- Filter by date range and tags
- Summary statistics
- Recent entries table

### 3. Add Entry
- Date picker
- Entry name and description
- Income/Expense type selector
- Amount and currency fields
- Tag management
- Form validation

### 4. Edit Entry
- Filter entries by date or tags
- Select entry to edit
- Update or delete entries
- Inline editing with validation

### 5. Export Ledger
- Optional filtering before export
- Export formats: JSON, CSV
- Preview before saving
- Save to file with custom path

## Building

```bash
# Development build
cargo build --bin beans

# Release build
cargo build --bin beans --release

# Run the application
cargo run --bin beans
```

## Dependencies

- **dioxus**: Desktop UI framework (v0.6)
- **dioxus-router**: Routing support
- **beans-lib**: Core ledger functionality
- **chrono**: Date/time handling
- **rust_decimal**: Precise decimal arithmetic

## Architecture

```
beans/src/
├── main.rs                 # Application entry point
├── app.rs                  # Main app component
├── state.rs                # Application state management
├── components/             # Reusable UI components
│   ├── mod.rs
│   ├── ribbon.rs          # Toolbar ribbon
│   ├── bar_chart.rs       # Income/expense visualization
│   ├── date_picker.rs     # Date selection
│   ├── entry_form.rs      # Add/edit entry form
│   └── filter_panel.rs    # Date/tag filtering
└── views/                  # Main application views
    ├── mod.rs
    ├── ledger_selection.rs  # Open/create ledger
    ├── overview.rs          # Dashboard with chart
    ├── add_entry.rs         # Add new entry
    ├── edit_entry.rs        # Edit existing entry
    └── export.rs            # Export functionality
```

## Styling

The UI uses custom CSS located in `assets/styles.css` with:
- Clean, modern design
- Responsive layouts
- Color-coded income (green) and expenses (red)
- Professional ribbon toolbar
- Intuitive form controls

## Development Notes

### State Management
- Uses Dioxus context/signals for reactive state
- Centralized `AppState` in `state.rs`
- Automatic UI updates on state changes

### Error Handling
- User-friendly error messages
- Success notifications for operations
- Input validation with clear feedback

### Integration with beans-lib
- All business logic handled by `beans-lib`
- UI is purely presentation layer
- Clean separation of concerns

## Future Enhancements

Potential improvements:
- File picker dialog for better UX
- Keyboard shortcuts
- Undo/redo functionality
- Multi-currency dashboard
- Advanced reporting visualizations
- Dark mode theme
- Import from other formats

## License

MIT License - see LICENSE file for details

