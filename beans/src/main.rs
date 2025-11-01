//! Beans - A multi-platform ledger application with Ribir UI
//!
//! This application provides a user interface for the beans-lib ledger library.
//! It allows users to:
//! - Create and open ledger files
//! - View income and expenses over time
//! - Add and edit ledger entries
//! - Export ledger data in various formats

mod app;
mod state;
mod ui;
mod utils;

use app::App;

fn main() {
    // Initialize logging
    env_logger::init();
    
    // Run the application
    App::run();
}
