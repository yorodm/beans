use beans_lib::ledger::LedgerManager;
use beans_lib::reporting::ReportGenerator;
use chrono::{DateTime, Utc};
use ribir::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

/// Represents the different views in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Initial view for opening or creating a ledger
    Welcome,
    /// Overview with income vs expenses chart
    Overview,
    /// Add new entry form
    AddEntry,
    /// Edit entry list and form
    EditEntry,
    /// Export ledger data
    Export,
}

/// Application state
pub struct AppState {
    /// Current active view
    pub current_view: Stateful<View>,
    /// Path to the current ledger file
    pub ledger_path: Stateful<Option<PathBuf>>,
    /// Ledger manager instance
    pub ledger: Stateful<Option<Arc<LedgerManager>>>,
    /// Report generator instance
    pub report_generator: Stateful<Option<Arc<ReportGenerator<'static>>>>,
    /// Selected date range for filtering
    pub date_range: Stateful<(DateTime<Utc>, DateTime<Utc>)>,
    /// Selected tags for filtering
    pub selected_tags: Stateful<Vec<String>>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        // Default to current month for date range
        let now = Utc::now();
        let start_of_month = Utc.with_ymd_and_hms(
            now.year(),
            now.month(),
            1,
            0,
            0,
            0,
        ).unwrap_or(now);
        
        Self {
            current_view: Stateful::new(View::Welcome),
            ledger_path: Stateful::new(None),
            ledger: Stateful::new(None),
            report_generator: Stateful::new(None),
            date_range: Stateful::new((start_of_month, now)),
            selected_tags: Stateful::new(Vec::new()),
        }
    }

    /// Open a ledger file
    pub async fn open_ledger(&self, path: PathBuf) -> anyhow::Result<()> {
        // Create a new ledger manager
        let ledger = LedgerManager::open(path.to_str().unwrap())?;
        let ledger_arc = Arc::new(ledger);
        
        // Create a report generator
        let report_gen = ReportGenerator::new(ledger_arc.as_ref());
        
        // Update state
        *self.ledger_path.write() = Some(path);
        *self.ledger.write() = Some(ledger_arc);
        *self.report_generator.write() = Some(Arc::new(report_gen));
        *self.current_view.write() = View::Overview;
        
        Ok(())
    }

    /// Create a new ledger file
    pub async fn create_ledger(&self, path: PathBuf) -> anyhow::Result<()> {
        // Create a new ledger manager
        let ledger = LedgerManager::create(path.to_str().unwrap())?;
        let ledger_arc = Arc::new(ledger);
        
        // Create a report generator
        let report_gen = ReportGenerator::new(ledger_arc.as_ref());
        
        // Update state
        *self.ledger_path.write() = Some(path);
        *self.ledger.write() = Some(ledger_arc);
        *self.report_generator.write() = Some(Arc::new(report_gen));
        *self.current_view.write() = View::Overview;
        
        Ok(())
    }

    /// Navigate to a different view
    pub fn navigate_to(&self, view: View) {
        *self.current_view.write() = view;
    }

    /// Update the date range for filtering
    pub fn update_date_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) {
        *self.date_range.write() = (start, end);
    }

    /// Update the selected tags for filtering
    pub fn update_selected_tags(&self, tags: Vec<String>) {
        *self.selected_tags.write() = tags;
    }
}
