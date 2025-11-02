//! Beans - Ledger application with Freya UI

mod app;
mod components;
mod state;
mod views;

use app::App;
use freya::prelude::*;

fn main() {
    // Initialize logger
    env_logger::init();
    
    // Launch the Freya application
    launch(
        App,
        WindowConfig {
            title: "Beans Ledger".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        },
    );
}
