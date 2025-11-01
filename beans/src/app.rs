use crate::{
    components::{error_dialog::ErrorDialog, ribbon::Ribbon},
    state::{AppState, AppView},
    views::{
        add_entry::AddEntry,
        edit_entry::EditEntry,
        export::Export,
        open_ledger::OpenLedger,
        overview::Overview,
    },
};
use freya::prelude::*;

/// Main application component
pub fn App() -> Element {
    // Create application state
    let state = AppState::new();
    
    // Get the current view
    let current_view = state.current_view.read();
    
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(245, 245, 245)",
            direction: "vertical",
            
            // Ribbon toolbar
            Ribbon { state: state.clone() }
            
            // Current view
            {
                match *current_view {
                    AppView::OpenLedger => rsx! { OpenLedger { state: state.clone() } },
                    AppView::Overview => rsx! { Overview { state: state.clone() } },
                    AppView::AddEntry => rsx! { AddEntry { state: state.clone() } },
                    AppView::EditEntry => rsx! { EditEntry { state: state.clone() } },
                    AppView::Export => rsx! { Export { state: state.clone() } },
                }
            }
            
            // Error dialog
            ErrorDialog { state: state.clone() }
        }
    }
}

