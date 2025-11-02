//! Entry form component for adding and editing ledger entries

use beans_lib::prelude::*;
use crate::components::date_picker::DatePicker;
use crate::styles;
use chrono::{TimeZone, Utc};
use freya::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

/// EntryForm component for adding and editing ledger entries
#[component]
pub fn EntryForm(
    entry: Option<LedgerEntry>,
    on_save: EventHandler<LedgerEntry>,
    on_cancel: EventHandler<()>,
) -> Element {
    // Form state
    let mut date = use_signal(|| {
        entry
            .as_ref()
            .map(|e| e.date().format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string())
    });
    
    let mut name = use_signal(|| entry.as_ref().map(|e| e.name().to_string()).unwrap_or_default());

    let mut entry_type = use_signal(|| {
        entry
            .as_ref()
            .map(|e| e.entry_type())
            .unwrap_or(EntryType::Expense)
    });

    let mut amount = use_signal(|| {
        entry
            .as_ref()
            .map(|e| e.amount().to_string())
            .unwrap_or_default()
    });

    let mut currency_code = use_signal(|| {
        entry
            .as_ref()
            .map(|e| e.currency_code())
            .unwrap_or_else(|| "USD".to_string())
    });

    let mut description = use_signal(|| {
        entry
            .as_ref()
            .and_then(|e| e.description().map(|d| d.to_string()))
            .unwrap_or_default()
    });

    let mut tag_input = use_signal(String::new);

    let mut tags = use_signal(|| {
        entry
            .as_ref()
            .map(|e| {
                e.tags()
                    .iter()
                    .map(|t| t.name().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    });

    // Error state
    let mut error_message = use_signal(String::new);

    // Add a tag to the list
    let add_tag = move |_| {
        let tag_name = tag_input().trim().to_string();
        if !tag_name.is_empty() {
            if !tag_name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                error_message.set("Tags can only contain letters, numbers, hyphens, and underscores".to_string());
                return;
            }

            let normalized = tag_name.to_lowercase();
            if !tags().iter().any(|t| t.to_lowercase() == normalized) {
                let mut new_tags = tags();
                new_tags.push(normalized);
                tags.set(new_tags);
            }

            tag_input.set(String::new());
            error_message.set(String::new());
        }
    };

    // Remove a tag from the list
    let mut remove_tag = move |idx: usize| {
        let mut new_tags = tags();
        if idx < new_tags.len() {
            new_tags.remove(idx);
            tags.set(new_tags);
        }
    };

    // Handle form submission
    let save = move |_| {
        error_message.set(String::new());

        // Validate required fields
        if name().trim().is_empty() {
            error_message.set("Name is required".to_string());
            return;
        }

        if amount().trim().is_empty() {
            error_message.set("Amount is required".to_string());
            return;
        }

        // Parse amount
        let amount_decimal = match Decimal::from_str(&amount()) {
            Ok(d) if d <= Decimal::ZERO => {
                error_message.set("Amount must be positive".to_string());
                return;
            }
            Ok(d) => d,
            Err(_) => {
                error_message.set("Invalid amount format".to_string());
                return;
            }
        };

        // Parse date
        let date_time = match chrono::NaiveDate::parse_from_str(&date(), "%Y-%m-%d") {
            Ok(d) => {
                let naive_datetime = d.and_hms_opt(0, 0, 0).unwrap();
                Utc.from_utc_datetime(&naive_datetime)
            }
            Err(_) => {
                error_message.set("Invalid date format".to_string());
                return;
            }
        };

        // Create tags
        let tag_objects = match tags()
            .iter()
            .map(|t| Tag::new(t))
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(tags) => tags,
            Err(e) => {
                error_message.set(format!("Invalid tag: {}", e));
                return;
            }
        };

        // Build the entry
        let mut builder = LedgerEntryBuilder::new()
            .name(name())
            .amount(amount_decimal)
            .currency_code(currency_code().to_uppercase())
            .entry_type(entry_type())
            .date(date_time)
            .tags(tag_objects);

        // Add description if present
        if !description().trim().is_empty() {
            builder = builder.description(description());
        }

        // If editing an existing entry, preserve its ID and creation timestamp
        if let Some(existing) = &entry {
            builder = builder.id(existing.id()).created_at(existing.created_at());
        }

        // Build the entry
        match builder.build() {
            Ok(entry) => on_save.call(entry),
            Err(e) => error_message.set(format!("Failed to build entry: {}", e)),
        }
    };

    let current_tags = tags();

    rsx! {
        ScrollView {
            width: "100%",
            height: "fill",
            show_scrollbar: true,

            rect {
                width: "100%",
                padding: "{styles::spacing::LARGE}",
                direction: "vertical",
                spacing: "{styles::spacing::LARGE}",

                // Error message
                if !error_message().is_empty() {
                    rect {
                        width: "100%",
                        padding: "{styles::spacing::MEDIUM}",
                        background: "{styles::colors::ERROR}",
                        corner_radius: "{styles::radius::SMALL}",

                        label {
                            color: "white",
                            font_size: "{styles::fonts::NORMAL}",
                            "{error_message()}"
                        }
                    }
                }

                // Date picker
                DatePicker {
                    label: "Date".to_string(),
                    value: date().clone(),
                    on_change: move |new_date| date.set(new_date)
                }

                // Name input
                rect {
                    direction: "vertical",
                    spacing: "{styles::spacing::SMALL}",
                    width: "100%",

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        "Name (required):"
                    }
                    Input {
                        value: name().clone(),
                        placeholder: "Entry name",
                        onchange: move |e| name.set(e),
                    }
                }

                // Entry type selector
                rect {
                    direction: "vertical",
                    spacing: "{styles::spacing::SMALL}",
                    width: "100%",

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        "Type:"
                    }

                    rect {
                        direction: "horizontal",
                        spacing: "{styles::spacing::MEDIUM}",

                        Button {
                            onclick: move |_| entry_type.set(EntryType::Income),
                            label {
                                color: if entry_type() == EntryType::Income { "white" } else { "{styles::colors::TEXT_PRIMARY}" },
                                "Income"
                            }
                        }

                        Button {
                            onclick: move |_| entry_type.set(EntryType::Expense),
                            label {
                                color: if entry_type() == EntryType::Expense { "white" } else { "{styles::colors::TEXT_PRIMARY}" },
                                "Expense"
                            }
                        }
                    }
                }

                // Amount input
                rect {
                    direction: "vertical",
                    spacing: "{styles::spacing::SMALL}",
                    width: "100%",

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        "Amount (required):"
                    }
                    Input {
                        value: amount().clone(),
                        placeholder: "0.00",
                        onchange: move |e| amount.set(e),
                    }
                }

                // Currency code input
                rect {
                    direction: "vertical",
                    spacing: "{styles::spacing::SMALL}",
                    width: "100%",

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        "Currency Code:"
                    }
                    Input {
                        value: currency_code().clone(),
                        placeholder: "USD",
                        onchange: move |e| currency_code.set(e.to_uppercase()),
                    }
                }

                // Description input
                rect {
                    direction: "vertical",
                    spacing: "{styles::spacing::SMALL}",
                    width: "100%",

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        "Description:"
                    }
                    Input {
                        value: description().clone(),
                        placeholder: "Optional description",
                        onchange: move |e| description.set(e),
                    }
                }

                // Tags management
                rect {
                    direction: "vertical",
                    spacing: "{styles::spacing::SMALL}",
                    width: "100%",

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        "Tags:"
                    }

                    rect {
                        direction: "horizontal",
                        spacing: "{styles::spacing::SMALL}",

                        Input {
                            value: tag_input().clone(),
                            placeholder: "Enter tag name",
                            onchange: move |e| tag_input.set(e),
                        }

                        Button {
                            onclick: move |_| add_tag(()),
                            label { "Add Tag" }
                        }
                    }

                    // Display current tags
                    if !current_tags.is_empty() {
                        rect {
                            direction: "horizontal",
                            spacing: "{styles::spacing::SMALL}",
                            padding: "{styles::spacing::SMALL}",

                            for (idx, tag) in current_tags.iter().enumerate() {
                                rect {
                                    direction: "horizontal",
                                    padding: "{styles::spacing::SMALL}",
                                    background: "{styles::colors::INFO}",
                                    corner_radius: "{styles::radius::SMALL}",
                                    spacing: "{styles::spacing::TINY}",
                                    cross_align: "center",

                                    label {
                                        color: "white",
                                        font_size: "{styles::fonts::SMALL}",
                                        "{tag}"
                                    }

                                    Button {
                                        onclick: move |_| remove_tag(idx),
                                        label { "×" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Action buttons
                rect {
                    direction: "horizontal",
                    spacing: "{styles::spacing::MEDIUM}",
                    main_align: "center",
                    padding: "{styles::spacing::LARGE} 0 0 0",

                    Button {
                        onclick: save,
                        label { "Save Entry" }
                    }

                    Button {
                        onclick: move |_| on_cancel.call(()),
                        label { "Cancel" }
                    }
                }
            }
        }
    }
}

