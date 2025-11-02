//! Filter panel component for date and tag filtering

use crate::state::AppState;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use dioxus::prelude::*;

#[component]
pub fn FilterPanel(on_apply: EventHandler<()>) -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    let mut start_date = use_signal(|| String::new());
    let mut end_date = use_signal(|| String::new());
    let mut tag_input = use_signal(|| String::new());

    // Initialize with current filter values
    use_effect(move || {
        let filter = &app_state.read().filter;

        if let Some(start) = filter.date_range.start {
            start_date.set(start.format("%Y-%m-%d").to_string());
        }

        if let Some(end) = filter.date_range.end {
            end_date.set(end.format("%Y-%m-%d").to_string());
        }
    });

    let apply_filter = move |_| {
        let mut state = app_state.write();

        // Parse start date
        if !start_date().is_empty() {
            if let Ok(naive_date) = NaiveDate::parse_from_str(&start_date(), "%Y-%m-%d") {
                state.filter.date_range.start =
                    Some(Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap()));
            }
        } else {
            state.filter.date_range.start = None;
        }

        // Parse end date
        if !end_date().is_empty() {
            if let Ok(naive_date) = NaiveDate::parse_from_str(&end_date(), "%Y-%m-%d") {
                state.filter.date_range.end =
                    Some(Utc.from_utc_datetime(&naive_date.and_hms_opt(23, 59, 59).unwrap()));
            }
        } else {
            state.filter.date_range.end = None;
        }

        // Reload entries with new filter
        if let Err(e) = state.load_entries() {
            state.set_error(format!("Failed to load entries: {}", e));
        }

        drop(state);
        on_apply.call(());
    };

    let mut add_tag = move |_| {
        if !tag_input().is_empty() {
            let mut state = app_state.write();
            let tag = tag_input().trim().to_string();
            if !state.filter.tags.contains(&tag) {
                state.filter.tags.push(tag);
            }
            drop(state);
            tag_input.set(String::new());
        }
    };

    let mut remove_tag = move |tag: String| {
        let mut state = app_state.write();
        state.filter.tags.retain(|t| t != &tag);
    };

    let clear_filter = move |_| {
        start_date.set(String::new());
        end_date.set(String::new());
        tag_input.set(String::new());

        let mut state = app_state.write();
        state.filter.date_range.start = None;
        state.filter.date_range.end = None;
        state.filter.tags.clear();

        if let Err(e) = state.load_entries() {
            state.set_error(format!("Failed to load entries: {}", e));
        }

        drop(state);
        on_apply.call(());
    };

    let current_tags = app_state.read().filter.tags.clone();

    rsx! {
        div {
            class: "filter-panel",

            h3 { "Filter Entries" }

            // Date range filters
            div {
                class: "filter-section",

                div {
                    class: "filter-row",

                    div {
                        class: "filter-field",
                        label { "Start Date:" }
                        input {
                            r#type: "date",
                            value: "{start_date}",
                            oninput: move |evt| start_date.set(evt.value().clone())
                        }
                    }

                    div {
                        class: "filter-field",
                        label { "End Date:" }
                        input {
                            r#type: "date",
                            value: "{end_date}",
                            oninput: move |evt| end_date.set(evt.value().clone())
                        }
                    }
                }
            }

            // Tag filters
            div {
                class: "filter-section",

                div {
                    class: "filter-row",

                    div {
                        class: "filter-field",
                        label { "Tags:" }
                        input {
                            r#type: "text",
                            value: "{tag_input}",
                            placeholder: "Enter tag name",
                            oninput: move |evt| tag_input.set(evt.value().clone()),
                        }
                        button {
                            onclick: move |evt| {
                                add_tag(())
                            },
                            "Add Tag"
                        }
                    }
                }

                // Display current tags
                if !current_tags.is_empty() {
                    div {
                        class: "tag-list",
                        for tag in current_tags {
                            div {
                                class: "tag-item",
                                span { "{tag}" }
                                button {
                                    class: "tag-remove",
                                    onclick: move |_| remove_tag(tag.clone()),
                                    "×"
                                }
                            }
                        }
                    }
                }
            }

            // Action buttons
            div {
                class: "filter-actions",

                button {
                    class: "button-primary",
                    onclick: apply_filter,
                    "Apply Filter"
                }

                button {
                    class: "button-secondary",
                    onclick: clear_filter,
                    "Clear Filter"
                }
            }
        }
    }
}
