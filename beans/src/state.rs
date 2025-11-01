use beans_lib::{
    ledger::LedgerManager,
    models::{Currency, EntryType, LedgerEntry, Tag},
    reporting::ReportGenerator,
    BeansResult,
};
use chrono::{DateTime, NaiveDate, Utc};
use dioxus_hooks::use_signal;
use freya::prelude::*;
use std::path::PathBuf;

/// Represents the different views in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    /// Open or create a new ledger
    OpenLedger,
    /// Overview of income vs expenses
    Overview,
    /// Add a new entry to the ledger
    AddEntry,
    /// Edit an existing entry
    EditEntry,
    /// Export the ledger
    Export,
}

/// Represents the filter options for entries
#[derive(Debug, Clone)]
pub struct EntryFilter {
    /// Start date for filtering
    pub start_date: Option<NaiveDate>,
    /// End date for filtering
    pub end_date: Option<NaiveDate>,
    /// Tags to filter by
    pub tags: Vec<String>,
    /// Currency to filter by
    pub currency: Option<Currency>,
    /// Entry type to filter by
    pub entry_type: Option<EntryType>,
}

impl Default for EntryFilter {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            tags: Vec::new(),
            currency: None,
            entry_type: None,
        }
    }
}

/// Represents the application state
pub struct AppState {
    /// Current view
    pub current_view: Signal<AppView>,
    /// Path to the current ledger file
    pub ledger_path: Signal<Option<PathBuf>>,
    /// Ledger manager instance
    pub ledger_manager: Signal<Option<LedgerManager>>,
    /// Report generator instance
    pub report_generator: Signal<Option<ReportGenerator>>,
    /// Current filter for entries
    pub filter: Signal<EntryFilter>,
    /// Selected entry for editing
    pub selected_entry: Signal<Option<LedgerEntry>>,
    /// Error message to display
    pub error_message: Signal<Option<String>>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            current_view: use_signal(|| AppView::OpenLedger),
            ledger_path: use_signal(|| None),
            ledger_manager: use_signal(|| None),
            report_generator: use_signal(|| None),
            filter: use_signal(|| EntryFilter::default()),
            selected_entry: use_signal(|| None),
            error_message: use_signal(|| None),
        }
    }

    /// Open a ledger file
    pub fn open_ledger(&self, path: PathBuf) -> BeansResult<()> {
        let manager = LedgerManager::open(&path)?;
        let report_gen = ReportGenerator::new(manager.clone());
        
        self.ledger_path.set(Some(path));
        self.ledger_manager.set(Some(manager));
        self.report_generator.set(Some(report_gen));
        self.current_view.set(AppView::Overview);
        
        Ok(())
    }

    /// Create a new ledger file
    pub fn create_ledger(&self, path: PathBuf) -> BeansResult<()> {
        let manager = LedgerManager::new(&path)?;
        let report_gen = ReportGenerator::new(manager.clone());
        
        self.ledger_path.set(Some(path));
        self.ledger_manager.set(Some(manager));
        self.report_generator.set(Some(report_gen));
        self.current_view.set(AppView::Overview);
        
        Ok(())
    }

    /// Set the current view
    pub fn set_view(&self, view: AppView) {
        // Only allow navigation to OpenLedger if no ledger is open
        // or to other views if a ledger is open
        match view {
            AppView::OpenLedger => {
                self.current_view.set(view);
            }
            _ => {
                if self.ledger_manager.read().is_some() {
                    self.current_view.set(view);
                } else {
                    self.error_message.set(Some("Please open a ledger first".to_string()));
                }
            }
        }
    }

    /// Apply the current filter
    pub fn apply_filter(&self, filter: EntryFilter) {
        self.filter.set(filter);
    }

    /// Select an entry for editing
    pub fn select_entry(&self, entry: LedgerEntry) {
        self.selected_entry.set(Some(entry));
        self.current_view.set(AppView::EditEntry);
    }

    /// Clear the selected entry
    pub fn clear_selected_entry(&self) {
        self.selected_entry.set(None);
    }

    /// Set an error message
    pub fn set_error(&self, message: String) {
        self.error_message.set(Some(message));
    }

    /// Clear the error message
    pub fn clear_error(&self) {
        self.error_message.set(None);
    }
}

