//! Beans - Ledger application with Freya UI

mod app;
mod components;
mod state;
mod views;

use app::App;
use freya::prelude::*;
use freya_native::EventLoop;

fn main() {
    // Initialize logger
    env_logger::init();
    
    // Launch the Freya application
    let event_loop = EventLoop::new().unwrap();
    
    launch_with_props(
        event_loop,
        App,
        (),
        WindowConfig {
            title: "Beans Ledger".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        },
    );
}
