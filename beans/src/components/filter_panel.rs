//! Filter panel component for date and tag filtering

use crate::state::AppState;
use crate::styles;
use chrono::{NaiveDate, TimeZone, Utc};
use freya::prelude::*;

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
        rect {
            width: "280",
            padding: "{styles::spacing::LARGE}",
            background: "white",
            corner_radius: "{styles::radius::MEDIUM}",
            shadow: "0 2 4 0 {styles::colors::SHADOW}",
            direction: "vertical",
            spacing: "{styles::spacing::LARGE}",

            // Title
            label {
                font_size: "{styles::fonts::LARGE}",
                font_weight: "bold",
                color: "{styles::colors::TEXT_PRIMARY}",
                "Filter Entries"
            }

            // Date range filters
            rect {
                direction: "vertical",
                spacing: "{styles::spacing::MEDIUM}",

                label {
                    font_size: "{styles::fonts::NORMAL}",
                    color: "{styles::colors::TEXT_PRIMARY}",
                    "Start Date:"
                }
                Input {
                    value: start_date.read().clone(),
                    placeholder: "YYYY-MM-DD",
                    onchange: move |e| start_date.set(e),
                }

                label {
                    font_size: "{styles::fonts::NORMAL}",
                    color: "{styles::colors::TEXT_PRIMARY}",
                    "End Date:"
                }
                Input {
                    value: end_date.read().clone(),
                    placeholder: "YYYY-MM-DD",
                    onchange: move |e| end_date.set(e),
                }
            }

            // Tag filters
            rect {
                direction: "vertical",
                spacing: "{styles::spacing::MEDIUM}",

                label {
                    font_size: "{styles::fonts::NORMAL}",
                    color: "{styles::colors::TEXT_PRIMARY}",
                    "Tags:"
                }
                
                rect {
                    direction: "horizontal",
                    spacing: "{styles::spacing::SMALL}",
                    
                    Input {
                        value: tag_input.read().clone(),
                        placeholder: "Enter tag name",
                        onchange: move |e| tag_input.set(e),
                    }
                    
                    Button {
                        onclick: move |_| add_tag(()),
                        label { "Add" }
                    }
                }

                // Display current tags
                if !current_tags.is_empty() {
                    ScrollView {
                        height: "120",
                        width: "100%",
                        show_scrollbar: true,
                        
                        rect {
                            direction: "vertical",
                            spacing: "{styles::spacing::SMALL}",
                            
                            for tag in current_tags {
                                rect {
                                    width: "100%",
                                    height: "30",
                                    direction: "horizontal",
                                    main_align: "spaceBetween",
                                    cross_align: "center",
                                    padding: "{styles::spacing::SMALL}",
                                    background: "{styles::colors::INFO}",
                                    corner_radius: "{styles::radius::SMALL}",
                                    
                                    label {
                                        color: "white",
                                        font_size: "{styles::fonts::SMALL}",
                                        "{tag}"
                                    }
                                    
                                    Button {
                                        onclick: move |_| remove_tag(tag.clone()),
                                        label { "×" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Action buttons
            rect {
                direction: "horizontal",
                spacing: "{styles::spacing::SMALL}",
                main_align: "center",

                Button {
                    onclick: apply_filter,
                    label { "Apply Filter" }
                }

                Button {
                    onclick: clear_filter,
                    label { "Clear Filter" }
                }
            }
        }
    }
}

