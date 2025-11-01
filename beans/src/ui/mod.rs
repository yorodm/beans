// UI module exports
pub mod ribbon;
pub mod welcome;
pub mod overview;
pub mod entry_form;
pub mod entry_list;
pub mod export;

// Re-export commonly used UI components
pub use ribbon::Ribbon;
pub use welcome::WelcomeView;
pub use overview::OverviewView;
pub use entry_form::EntryFormView;
pub use entry_list::EntryListView;
pub use export::ExportView;
