//! Ribbon toolbar component

use crate::state::{AppState, View};
use dioxus::prelude::*;

#[component]
pub fn Ribbon() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let has_ledger = app_state.read().ledger_manager.is_some();

    rsx! {
        div {
            class: "ribbon",

            button {
                class: "ribbon-button",
                onclick: move |_| {
                    app_state.write().set_view(View::LedgerSelection);
                },
                "📂 Open/Create Ledger"
            }

            div { class: "ribbon-separator" }

            button {
                class: "ribbon-button",
                disabled: !has_ledger,
                onclick: move |_| {
                    app_state.write().set_view(View::Overview);
                },
                "📊 Overview"
            }

            button {
                class: "ribbon-button",
                disabled: !has_ledger,
                onclick: move |_| {
                    app_state.write().set_view(View::AddEntry);
                },
                "➕ Add Entry"
            }

            button {
                class: "ribbon-button",
                disabled: !has_ledger,
                onclick: move |_| {
                    app_state.write().set_view(View::EditEntry);
                },
                "✏️ Edit Entry"
            }

            button {
                class: "ribbon-button",
                disabled: !has_ledger,
                onclick: move |_| {
                    app_state.write().set_view(View::ExportLedger);
                },
                "💾 Export Ledger"
            }

            // Show current ledger name if one is open
            if let Some(path) = &app_state.read().ledger_path {
                div {
                    class: "ledger-name",
                    "Current: {path.file_name().unwrap_or_default().to_string_lossy()}"
                }
            }
        }
    }
}
