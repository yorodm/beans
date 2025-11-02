//! Main application component

use crate::components::ribbon::Ribbon;
use crate::state::{AppState, View};
use crate::styles;
use crate::views;
use freya::prelude::*;

#[component]
pub fn App() -> Element {
    let app_state = use_context_provider(|| Signal::new(AppState::new()));

    rsx! {
        Body {
            background: "{styles::colors::BACKGROUND}",
            padding: "0",
            width: "100%",
            height: "100%",

            // Ribbon toolbar at the top
            Ribbon {}

            // Main content area
            rect {
                width: "100%",
                height: "fill",
                padding: "{styles::spacing::LARGE}",

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
