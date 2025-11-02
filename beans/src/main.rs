//! Beans - Ledger application with Freya UI

mod app;
mod components;
mod state;
mod styles;
mod views;

fn main() {
    env_logger::init();
    log::info!("Starting Beans ledger application");

    freya::launch_cfg(
        app::App,
        freya::WindowConfig::builder()
            .with_title("Beans - Ledger Manager")
            .with_width(1200.0)
            .with_height(800.0)
            .with_decorations(true)
            .with_transparency(false)
            .build(),
    );
}
