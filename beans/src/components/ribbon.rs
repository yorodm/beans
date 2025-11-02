//! Ribbon toolbar component

use crate::state::{AppState, View};
use crate::styles;
use freya::prelude::*;

#[component]
pub fn Ribbon() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let has_ledger = app_state.read().ledger_manager.is_some();

    rsx! {
        rect {
            width: "100%",
            height: "60",
            background: "{styles::colors::RIBBON_BG}",
            padding: "{styles::spacing::MEDIUM}",
            direction: "horizontal",
            main_align: "start",
            cross_align: "center",
            spacing: "{styles::spacing::SMALL}",
            shadow: "0 2 4 0 {styles::colors::SHADOW}",

            Button {
                onclick: move |_| {
                    app_state.write().set_view(View::LedgerSelection);
                },
                label { "📂 Open/Create Ledger" }
            }

            rect {
                width: "2",
                height: "30",
                background: "{styles::colors::BORDER}",
                margin: "0 {styles::spacing::SMALL}",
            }

            Button {
                enabled: has_ledger,
                onclick: move |_| {
                    app_state.write().set_view(View::Overview);
                },
                label { "📊 Overview" }
            }

            Button {
                enabled: has_ledger,
                onclick: move |_| {
                    app_state.write().set_view(View::AddEntry);
                },
                label { "➕ Add Entry" }
            }

            Button {
                enabled: has_ledger,
                onclick: move |_| {
                    app_state.write().set_view(View::EditEntry);
                },
                label { "✏️ Edit Entry" }
            }

            Button {
                enabled: has_ledger,
                onclick: move |_| {
                    app_state.write().set_view(View::ExportLedger);
                },
                label { "💾 Export Ledger" }
            }

            Button {
                onclick: move |_| {
                    std::process::exit(0)
                },
                label { "❌ Exit" }
            }

            // Show current ledger name if one is open
            if let Some(path) = &app_state.read().ledger_path {
                label {
                    color: "{styles::colors::TEXT_SECONDARY}",
                    font_size: "{styles::fonts::NORMAL}",
                    margin: "0 0 0 {styles::spacing::LARGE}",
                    "Current: {path.file_name().unwrap_or_default().to_string_lossy()}"
                }
            }
        }
    }
}
