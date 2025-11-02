//! Main application component

use crate::components::ribbon::Ribbon;
use crate::state::{AppState, View};
use crate::views;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let app_state = use_context_provider(|| Signal::new(AppState::new()));

    rsx! {
        style { {include_str!("../assets/styles.css")} }

        div {
            class: "app-container",

            // Ribbon toolbar at the top
            Ribbon {}

            // Main content area
            div {
                class: "content-area",

                match app_state.read().current_view {
                    View::LedgerSelection => rsx! { views::ledger_selection::LedgerSelectionView {} },
                    View::Overview => rsx! { views::overview::OverviewView {} },
                    View::AddEntry => rsx! { views::add_entry::AddEntryView {} },
                    View::EditEntry => rsx! { views::edit_entry::EditEntryView {} },
                    View::ExportLedger => rsx! { views::export::ExportView {} },
                }
            }
        }
    }
}
