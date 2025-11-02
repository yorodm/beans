//! Beans - Ledger application with Dioxus UI

use dioxus::prelude::*;

mod app;
mod components;
mod state;
mod views;

fn main() {
    env_logger::init();
    log::info!("Starting Beans ledger application");

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new().with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("Beans - Ledger Manager")
                    .with_resizable(true)
                    .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1200.0, 800.0)),
            ),
        )
        .launch(app::App);
}
