# Beans UI Implementation Guide

## Overview

The Beans UI has been implemented using the **Dioxus** framework (v0.6) as requested, providing a clean desktop application interface for managing ledger files.

## Implementation Status

✅ **Complete** - All requested features have been implemented:

### 1. Ribbon Toolbar
- **Open/Create Ledger**: Opens file dialog view
- **Overview**: Dashboard with income/expense visualization
- **Add Entry**: Form for creating new entries
- **Edit Entry**: Entry selection and editing interface
- **Export Ledger**: Export to supported formats

All buttons are disabled until a ledger is opened (except Open/Create).

### 2. Views

#### Ledger Selection View
- Two-card layout for opening vs creating ledgers
- Path input with automatic `.bean` extension handling
- Error handling for invalid paths or permissions
- Success messages on successful operations

#### Overview View
- **Date display**: Shows current date
- **Bar chart**: Visual comparison of income (green) vs expenses (red)
  - Bars are properly labeled
  - Values displayed on bars
  - Summary statistics below chart
- **Filter panel**: Left sidebar with:
  - Start/End date pickers
  - Tag filtering with add/remove
  - Apply and Clear buttons
- **Entry list**: Table showing recent entries (up to 10)
- **Empty state**: Helpful message when no entries exist

#### Add Entry View
- Comprehensive form with all fields:
  - Date picker
  - Name (required)
  - Type selector (Income/Expense)
  - Amount (validated)
  - Currency code (3-letter, auto-uppercase)
  - Description (optional, textarea)
  - Tag management (add/remove)
- Form validation with error messages
- Cancel button returns to Overview
- Auto-navigation to Overview on success

#### Edit Entry View
- **Two-stage interface**:
  1. Entry selection with filter panel
  2. Edit form (same as Add Entry)
- Filter by date range or tags before selecting
- Table view of filtered entries
- Edit or Delete actions per entry
- Confirmation on delete (via success message)
- Returns to selection view after save/cancel

#### Export View
- **Three-panel layout**:
  1. Filter configuration (optional)
  2. Format selection (JSON/CSV radio buttons)
  3. Preview and save area
- Generate button creates report with current filters
- Preview shows full report content
- Save to file with custom path
- Support for both JSON and CSV formats
- Help text explaining format options

## Technical Details

### Architecture

```
Dioxus Desktop App
    ↓
App Component (app.rs)
    ↓
Ribbon + Current View
    ↓
Views use Components
    ↓
All interact with AppState
    ↓
AppState uses beans-lib
```

### State Management

- **AppState** (state.rs): Central state store
  - Current view tracking
  - Ledger manager instance
  - Filtered entries cache
  - Filter configuration
  - Error/success messages
  
- **Dioxus Signals**: Reactive state updates
  - Context provider pattern
  - Automatic UI updates on state changes
  
### Component Architecture

**Reusable Components**:
- `Ribbon`: Top toolbar with navigation buttons
- `BarChart`: Income vs expenses visualization
- `DatePicker`: Date selection widget
- `FilterPanel`: Combined date/tag filtering
- `EntryForm`: Shared form for add/edit

**Views** (Full pages):
- Each view is self-contained
- Uses components where appropriate
- Manages local UI state with signals
- Interacts with AppState for data operations

### Styling

- **Custom CSS** in `assets/styles.css`
- Modern, clean design with:
  - Gradient ribbon toolbar
  - Card-based layouts
  - Color-coded income/expenses
  - Responsive grids
  - Professional form controls
  - Hover states and transitions

### Integration with beans-lib

The UI delegates all business logic to `beans-lib`:
- **LedgerManager**: File operations
- **EntryFilter**: Query building
- **ReportGenerator**: Export functionality
- **Currency, Tag, Entry models**: Data structures

No business logic in UI layer - pure presentation.

## Key Design Decisions

### 1. Dioxus Desktop over Web
- Native desktop application
- No web server required
- Better file system integration
- More native feel

### 2. Single Window Design
- Ribbon toolbar always visible
- Content area switches views
- Consistent navigation model
- No modal dialogs (inline forms instead)

### 3. Filter-First Pattern
- Filters available in relevant views
- Apply filters before operations
- Visual feedback on active filters
- Easy to clear and reapply

### 4. Optimistic UI
- Show success messages
- Navigate automatically on success
- Reload data after operations
- Clear messages between operations

### 5. Validation Strategy
- Client-side validation in forms
- Leverage bean-lib validation
- Clear error messages
- Prevent invalid submissions

## Building and Running

### Prerequisites
- Rust toolchain (1.70+)
- System dependencies for Dioxus desktop

### Build Commands
```bash
# Check compilation
cargo check --bin beans

# Development build
cargo build --bin beans

# Release build (optimized)
cargo build --bin beans --release

# Run directly
cargo run --bin beans

# Run release version
cargo run --bin beans --release
```

### Development with Nix (if available)
```bash
nix develop
cargo run --bin beans
```

## Testing the UI

### Manual Test Flow

1. **Start application**
   ```bash
   cargo run --bin beans
   ```

2. **Create a ledger**
   - Enter path: `test_ledger.bean`
   - Click "Create Ledger"
   - Should navigate to Overview

3. **Add entries**
   - Click "Add Entry" in ribbon
   - Fill form (date, name, amount, etc.)
   - Add some tags
   - Save
   - Should return to Overview

4. **View Overview**
   - Should see bar chart
   - Try filtering by date
   - Try filtering by tags
   - Check entry count updates

5. **Edit entries**
   - Click "Edit Entry"
   - Select an entry
   - Modify fields
   - Save or delete
   - Verify changes

6. **Export**
   - Click "Export Ledger"
   - Apply filters (optional)
   - Select format (JSON/CSV)
   - Generate report
   - Preview content
   - Save to file

## Known Limitations

1. **No file picker dialog**: Users must type paths manually
   - Could be added with native file dialog crate
   
2. **No drag-and-drop**: For opening ledger files
   - Future enhancement possibility
   
3. **Basic error handling**: Errors shown as messages
   - Could add toast notifications
   
4. **No undo/redo**: Operations are immediate
   - Could implement command pattern
   
5. **Single ledger at a time**: No tabs or multiple windows
   - Design decision for simplicity

## Future Enhancement Ideas

- **File dialogs**: Native open/save dialogs
- **Keyboard shortcuts**: Ctrl+N, Ctrl+O, etc.
- **Multi-window**: Open multiple ledgers
- **Advanced charts**: Line charts, pie charts, trends
- **Search**: Full-text search across entries
- **Import**: From CSV, QIF, OFX
- **Backup**: Auto-backup on changes
- **Dark mode**: Theme switching
- **Localization**: Multiple language support
- **Drag-and-drop**: For opening files
- **Recent files**: Quick access to recent ledgers

## Maintenance Notes

### Adding New Fields to Entries
1. Update `EntryForm` component
2. Update `entry` table view in relevant views
3. No changes needed to state management (uses bean-lib models)

### Adding New Views
1. Create view file in `views/`
2. Add to `views/mod.rs`
3. Add enum variant to `View` in `state.rs`
4. Add ribbon button in `ribbon.rs`
5. Add case in `app.rs` match statement

### Modifying Filters
1. Update `Filter` struct in `state.rs`
2. Update `FilterPanel` component
3. Update `load_entries` method to use new filters

## Caveats per Instructions

- ✅ **No modifications to beans-lib**: All changes in `beans/` directory only
- ✅ **Uses Dioxus framework**: As requested
- ✅ **Single window with ribbon**: Implemented
- ✅ **All views accessible via ribbon**: Complete
- ✅ **Bar chart with labels**: Green income, red expenses
- ✅ **Filter by date and tags**: In Overview and Edit Entry
- ✅ **Export to supported formats**: JSON and CSV via beans-lib

## Summary

The Beans UI is a complete, functional desktop application that provides:
- Intuitive ledger file management
- Visual income/expense tracking
- Powerful filtering capabilities
- Full CRUD operations on entries
- Professional export functionality

All implemented using Dioxus as requested, with clean separation between UI (beans) and business logic (beans-lib).

Ready for testing and use! 🫘✨

