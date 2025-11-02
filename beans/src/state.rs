//! Application state management

use beans_lib::prelude::*;
use chrono::{DateTime, Utc};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    LedgerSelection,
    Overview,
    AddEntry,
    EditEntry,
    ExportLedger,
}

#[derive(Debug, Clone)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

impl Default for DateRange {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub date_range: DateRange,
    pub tags: Vec<String>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            date_range: DateRange::default(),
            tags: Vec::new(),
        }
    }
}

pub struct AppState {
    pub current_view: View,
    pub ledger_manager: Option<LedgerManager>,
    pub ledger_path: Option<PathBuf>,
    pub entries: Vec<LedgerEntry>,
    pub filter: Filter,
    pub selected_entry: Option<Uuid>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: View::LedgerSelection,
            ledger_manager: None,
            ledger_path: None,
            entries: Vec::new(),
            filter: Filter::default(),
            selected_entry: None,
            error_message: None,
            success_message: None,
        }
    }
    
    pub fn set_view(&mut self, view: View) {
        self.current_view = view;
        self.clear_messages();
    }
    
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }
    
    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }
    
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }
    
    pub fn load_entries(&mut self) -> BeansResult<()> {
        if let Some(manager) = &self.ledger_manager {
            let mut filter_obj = EntryFilter::new();
            
            if let Some(start) = self.filter.date_range.start {
                filter_obj = filter_obj.with_start_date(start);
            }
            
            if let Some(end) = self.filter.date_range.end {
                filter_obj = filter_obj.with_end_date(end);
            }
            
            for tag in &self.filter.tags {
                filter_obj = filter_obj.with_tag(tag);
            }
            
            self.entries = manager.get_entries(Some(filter_obj))?;
        }
        Ok(())
    }
    
    pub fn open_ledger(&mut self, path: PathBuf) -> BeansResult<()> {
        let manager = LedgerManager::open(path.clone())?;
        self.ledger_manager = Some(manager);
        self.ledger_path = Some(path);
        self.load_entries()?;
        self.set_view(View::Overview);
        Ok(())
    }
    
    pub fn create_ledger(&mut self, path: PathBuf) -> BeansResult<()> {
        let manager = LedgerManager::create(path.clone())?;
        self.ledger_manager = Some(manager);
        self.ledger_path = Some(path);
        self.entries = Vec::new();
        self.set_view(View::Overview);
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

