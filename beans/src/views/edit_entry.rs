//! Edit entry view for modifying or deleting existing entries

use crate::components::entry_form::EntryForm;
use crate::components::filter_panel::FilterPanel;
use crate::state::{AppState, View};
use beans_lib::prelude::*;
use dioxus::prelude::*;

/// Edit Entry View
///
/// This view provides a two-stage interface:
/// 1. Selection stage: Filter and list entries, select one to edit
/// 2. Edit stage: Use EntryForm to modify the selected entry
#[component]
pub fn EditEntryView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    // Local state to track if we're in selection or edit mode
    let mut editing = use_signal(|| false);

    // Get the selected entry if any
    let selected_entry = {
        let state = app_state.read();
        if let Some(id) = state.selected_entry {
            state.entries.iter().find(|e| e.id() == id).cloned()
        } else {
            None
        }
    };

    // If we have a selected entry, go to edit mode
    if selected_entry.is_some() && !editing() {
        editing.set(true);
    }

    // Handle filter apply
    let on_filter_apply = move |_| {
        // Filtering is handled by the FilterPanel component
        // We just need to refresh the view
    };

    // Handle entry selection
    let mut select_entry = move |id: Uuid| {
        let mut state = app_state.write();
        state.selected_entry = Some(id);
        drop(state);
        editing.set(true);
    };

    // Handle save action
    let on_save = move |entry: LedgerEntry| {
        let mut state = app_state.write();

        // Update the entry in the ledger
        if let Some(manager) = &state.ledger_manager {
            match manager.update_entry(&entry) {
                Ok(_) => {
                    // Set success message
                    state.set_success("Entry updated successfully".to_string());

                    // Reload entries
                    if let Err(e) = state.load_entries() {
                        state.set_error(format!("Failed to reload entries: {}", e));
                    }

                    // Clear selection and go back to selection mode
                    state.selected_entry = None;
                    drop(state);
                    editing.set(false);
                }
                Err(e) => {
                    state.set_error(format!("Failed to update entry: {}", e));
                }
            }
        } else {
            state.set_error("No ledger is open".to_string());
        }
    };

    // Handle delete action
    let mut delete_entry = move |id: Uuid| {
        let mut state = app_state.write();

        // Delete the entry from the ledger
        if let Some(manager) = &state.ledger_manager {
            match manager.delete_entry(id) {
                Ok(_) => {
                    // Set success message
                    state.set_success("Entry deleted successfully".to_string());

                    // Reload entries
                    if let Err(e) = state.load_entries() {
                        state.set_error(format!("Failed to reload entries: {}", e));
                    }

                    // Clear selection and go back to selection mode
                    state.selected_entry = None;
                    drop(state);
                    editing.set(false);
                }
                Err(e) => {
                    state.set_error(format!("Failed to delete entry: {}", e));
                }
            }
        } else {
            state.set_error("No ledger is open".to_string());
        }
    };

    // Handle cancel action
    let on_cancel = move |_| {
        // Clear selection and go back to selection mode
        app_state.write().selected_entry = None;
        editing.set(false);
    };

    // Handle back to overview
    let back_to_overview = move |_| {
        let mut state = app_state.write();
        state.selected_entry = None;
        state.set_view(View::Overview);
    };
    let entries = app_state.read().entries.clone().into_iter();

    rsx! {
        div {
            class: "view edit-entry-view",

            // Header
            div {
                class: "view-header",
                h1 { if editing() { "Edit Entry" } else { "Select Entry to Edit" } }

                button {
                    class: "button-secondary back-button",
                    onclick: back_to_overview,
                    "Back to Overview"
                }
            }

            // Success/error messages
            {
                if let Some(success) = &app_state.read().success_message {
                    rsx! {
                        div {
                            class: "success-message",
                            "{success}"
                        }
                    }
                } else {
                    rsx!{}
                }
            }

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

            // Main content - either selection or edit form
            if editing() {
                // Edit form
                if let Some(entry) = selected_entry {
                    div {
                        class: "edit-container",

                        // Entry form
                        EntryForm {
                            entry: Some(entry.clone()),
                            on_save: on_save,
                            on_cancel: on_cancel
                        }

                        // Delete button
                        div {
                            class: "delete-container",
                            button {
                                class: "button-danger",
                                onclick: move |_| delete_entry(entry.id()),
                                "Delete Entry"
                            }
                        }
                    }
                } else {
                    // This shouldn't happen, but just in case
                    div {
                        class: "error-message",
                        "No entry selected. Please go back and select an entry."
                    }

                    button {
                        class: "button-secondary",
                        onclick: move |_| editing.set(false),
                        "Back to Selection"
                    }
                }
            } else {
                // Selection view
                div {
                    class: "selection-container",

                    // Left sidebar with filters
                    div {
                        class: "selection-sidebar",
                        FilterPanel {
                            on_apply: on_filter_apply
                        }
                    }

                    // Main area with entries list
                    div {
                        class: "selection-main",

                        if app_state.read().entries.is_empty() {
                            div {
                                class: "empty-state",
                                p { "No entries found. Try adjusting your filters or add a new entry." }
                                button {
                                    class: "button-primary",
                                    onclick: move |_| app_state.write().set_view(View::AddEntry),
                                    "Add Entry"
                                }
                            }
                        } else {
                            table {
                                class: "entries-table",

                                thead {
                                    tr {
                                        th { "Date" }
                                        th { "Name" }
                                        th { "Type" }
                                        th { "Amount" }
                                        th { "Currency" }
                                        th { "Tags" }
                                        th { "Actions" }
                                    }
                                }

                                tbody {
                                    for entry in entries {
                                        tr {
                                            class: match entry.entry_type() {
                                                EntryType::Income => "income-row",
                                                EntryType::Expense => "expense-row",
                                            },

                                            td { "{entry.date().format(\"%Y-%m-%d\")}" }
                                            td { "{entry.name()}" }
                                            td { "{entry.entry_type()}" }
                                            td { "{entry.amount()}" }
                                            td { "{entry.currency_code()}" }
                                            td {
                                                class: "tag-cell",
                                                for tag in entry.tags() {
                                                    span { class: "tag-pill", "{tag.name()}" }
                                                }
                                            }
                                            td {
                                                div {
                                                    class: "action-buttons",

                                                    button {
                                                        class: "button-small",
                                                        onclick: {
                                                            let id = entry.id();
                                                            move |_| select_entry(id)
                                                        },
                                                        "Edit"
                                                    }

                                                    button {
                                                        class: "button-small button-danger",
                                                        onclick: {
                                                            let id = entry.id();
                                                            move |_| delete_entry(id)
                                                        },
                                                        "Delete"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
