//! Edit entry view for modifying or deleting existing entries

use crate::components::entry_form::EntryForm;
use crate::components::filter_panel::FilterPanel;
use crate::state::{AppState, View};
use crate::styles;
use beans_lib::prelude::*;
use freya::prelude::*;

/// Edit Entry View - Two-stage interface for selecting and editing entries
#[component]
pub fn EditEntryView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
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
    let on_filter_apply = move |_| {};

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

        if let Some(manager) = &state.ledger_manager {
            match manager.update_entry(&entry) {
                Ok(_) => {
                    state.set_success("Entry updated successfully".to_string());
                    if let Err(e) = state.load_entries() {
                        state.set_error(format!("Failed to reload entries: {}", e));
                    }
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

        if let Some(manager) = &state.ledger_manager {
            match manager.delete_entry(id) {
                Ok(_) => {
                    state.set_success("Entry deleted successfully".to_string());
                    if let Err(e) = state.load_entries() {
                        state.set_error(format!("Failed to reload entries: {}", e));
                    }
                    state.selected_entry = None;
                    drop(state);
                    editing.set(false);
                }
                Err(e) => {
                    state.set_error(format!("Failed to delete entry: {}", e));
                }
            }
        }
    };

    // Handle cancel action
    let on_cancel = move |_| {
        app_state.write().selected_entry = None;
        editing.set(false);
    };

    let entries = app_state.read().entries.clone();

    rsx! {
        rect {
            width: "100%",
            height: "fill",
            padding: "{styles::spacing::LARGE}",
            direction: "vertical",
            spacing: "{styles::spacing::LARGE}",

            label {
                font_size: "{styles::fonts::TITLE}",
                font_weight: "bold",
                color: "{styles::colors::TEXT_PRIMARY}",
                "Edit Entry"
            }

            // Messages
            if let Some(success) = &app_state.read().success_message {
                rect {
                    width: "100%",
                    padding: "{styles::spacing::MEDIUM}",
                    background: "{styles::colors::SUCCESS}",
                    corner_radius: "{styles::radius::SMALL}",
                    label {
                        color: "white",
                        font_size: "{styles::fonts::NORMAL}",
                        "{success}"
                    }
                }
            }

            if let Some(error) = &app_state.read().error_message {
                rect {
                    width: "100%",
                    padding: "{styles::spacing::MEDIUM}",
                    background: "{styles::colors::ERROR}",
                    corner_radius: "{styles::radius::SMALL}",
                    label {
                        color: "white",
                        font_size: "{styles::fonts::NORMAL}",
                        "{error}"
                    }
                }
            }

            // Show either selection view or edit form
            if editing() && selected_entry.is_some() {
                EntryForm {
                    entry: selected_entry.clone(),
                    on_save: on_save,
                    on_cancel: on_cancel
                }

                Button {
                    onclick: move |_| {
                        if let Some(entry) = &selected_entry {
                            delete_entry(entry.id());
                        }
                    },
                    label { "Delete Entry" }
                }
            } else {
                // Selection view
                rect {
                    width: "100%",
                    direction: "horizontal",
                    spacing: "{styles::spacing::LARGE}",

                    FilterPanel {
                        on_apply: on_filter_apply
                    }

                    // Entry list
                    ScrollView {
                        width: "fill",
                        height: "600",
                        show_scrollbar: true,

                        rect {
                            direction: "vertical",
                            spacing: "{styles::spacing::SMALL}",

                            if entries.is_empty() {
                                label {
                                    color: "{styles::colors::TEXT_SECONDARY}",
                                    "No entries found."
                                }
                            } else {
                                for entry in entries.iter() {
                                    rect {
                                        width: "100%",
                                        padding: "{styles::spacing::MEDIUM}",
                                        background: "white",
                                        corner_radius: "{styles::radius::SMALL}",
                                        shadow: "0 1 3 0 {styles::colors::SHADOW}",
                                        direction: "horizontal",
                                        main_align: "spaceBetween",
                                        cross_align: "center",

                                        rect {
                                            direction: "vertical",
                                            spacing: "{styles::spacing::TINY}",

                                            label {
                                                font_size: "{styles::fonts::NORMAL}",
                                                font_weight: "bold",
                                                color: "{styles::colors::TEXT_PRIMARY}",
                                                "{entry.name()}"
                                            }

                                            label {
                                                font_size: "{styles::fonts::SMALL}",
                                                color: "{styles::colors::TEXT_SECONDARY}",
                                                "{entry.date().format(\"%Y-%m-%d\")} • {entry.amount()} {entry.currency_code()}"
                                            }
                                        }

                                        Button {
                                            onclick: move |_| select_entry(entry.id()),
                                            label { "Edit" }
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

