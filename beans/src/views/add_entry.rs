//! Add entry view for creating new ledger entries

use crate::components::entry_form::EntryForm;
use crate::state::{AppState, View};
use beans_lib::prelude::*;
use dioxus::prelude::*;

/// Add Entry View
///
/// This view allows users to create new ledger entries using the EntryForm component.
/// On successful save, it navigates back to the Overview view.
#[component]
pub fn AddEntryView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    // Handle save action
    let on_save = move |entry: LedgerEntry| {
        let mut state = app_state.write();

        // Add the entry to the ledger
        if let Some(manager) = &state.ledger_manager {
            match manager.add_entry(&entry) {
                Ok(_) => {
                    // Set success message
                    state.set_success("Entry added successfully".to_string());

                    // Reload entries
                    if let Err(e) = state.load_entries() {
                        state.set_error(format!("Failed to reload entries: {}", e));
                    }

                    // Navigate back to overview
                    state.set_view(View::Overview);
                }
                Err(e) => {
                    state.set_error(format!("Failed to add entry: {}", e));
                }
            }
        } else {
            state.set_error("No ledger is open".to_string());
        }
    };

    // Handle cancel action
    let on_cancel = move |_| {
        app_state.write().set_view(View::Overview);
    };

    rsx! {
        div {
            class: "view add-entry-view",

            h1 { "Add New Entry" }

            // Display error message if any
            {
                if let Some(error) = &app_state.read().error_message {
                    rsx! {
                        div {
                            class: "error-message",
                            "{error}"
                        }
                    }
                } else {
                    rsx!{}
                }
            }

            // Entry form
            EntryForm {
                entry: None,
                on_save: on_save,
                on_cancel: on_cancel
            }
        }
    }
}
