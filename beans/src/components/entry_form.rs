//! Entry form component for adding and editing ledger entries

use beans_lib::prelude::*;
use dioxus::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::components::date_picker::DatePicker;

/// EntryForm component for adding and editing ledger entries
/// 
/// This component provides a form for creating or editing ledger entries with:
/// - Date picker
/// - Name input (required)
/// - Type selector (Income/Expense radio buttons)
/// - Amount input with validation
/// - Currency code input (3-letter, auto-uppercase)
/// - Description textarea
/// - Tag management (add/remove tags)
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
            // Validate tag format (only letters, numbers, hyphens, underscores)
            if !tag_name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                error_message.set("Tags can only contain letters, numbers, hyphens, and underscores".to_string());
                return;
            }
            
            // Check for spaces
            if tag_name.contains(' ') {
                error_message.set("Tags cannot contain spaces".to_string());
                return;
            }
            
            // Add tag if it doesn't already exist
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
    let remove_tag = move |idx: usize| {
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
        
        if currency_code().trim().is_empty() {
            error_message.set("Currency code is required".to_string());
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
            Ok(entry) => {
                on_save.call(entry);
            }
            Err(e) => {
                error_message.set(format!("Error: {}", e));
            }
        }
    };
    
    rsx! {
        div {
            class: "entry-form",
            
            // Error message
            {
                if !error_message().is_empty() {
                    rsx! {
                        div {
                            class: "error-message",
                            "{error_message}"
                        }
                    }
                }
            }
            
            // Form fields
            div {
                class: "form-grid",
                
                // Date
                div {
                    class: "form-field",
                    DatePicker {
                        label: "Date".into(),
                        value: date(),
                        on_change: move |new_date| date.set(new_date)
                    }
                }
                
                // Name
                div {
                    class: "form-field",
                    label {
                        class: "form-label",
                        "Name *"
                    }
                    input {
                        class: "form-input",
                        r#type: "text",
                        value: "{name}",
                        placeholder: "Enter entry name",
                        oninput: move |evt| name.set(evt.value().clone())
                    }
                }
                
                // Entry Type
                div {
                    class: "form-field",
                    label {
                        class: "form-label",
                        "Type *"
                    }
                    div {
                        class: "radio-group",
                        
                        label {
                            class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "entry-type",
                                checked: entry_type() == EntryType::Income,
                                oninput: move |_| entry_type.set(EntryType::Income)
                            }
                            "Income"
                        }
                        
                        label {
                            class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "entry-type",
                                checked: entry_type() == EntryType::Expense,
                                oninput: move |_| entry_type.set(EntryType::Expense)
                            }
                            "Expense"
                        }
                    }
                }
                
                // Amount
                div {
                    class: "form-field",
                    label {
                        class: "form-label",
                        "Amount *"
                    }
                    input {
                        class: "form-input",
                        r#type: "number",
                        value: "{amount}",
                        placeholder: "0.00",
                        step: "0.01",
                        min: "0.01",
                        oninput: move |evt| amount.set(evt.value().clone())
                    }
                }
                
                // Currency
                div {
                    class: "form-field",
                    label {
                        class: "form-label",
                        "Currency Code *"
                    }
                    input {
                        class: "form-input",
                        r#type: "text",
                        value: "{currency_code}",
                        placeholder: "USD",
                        maxlength: "3",
                        oninput: move |evt| {
                            let value = evt.value().clone().to_uppercase();
                            currency_code.set(value);
                        }
                    }
                }
                
                // Description
                div {
                    class: "form-field full-width",
                    label {
                        class: "form-label",
                        "Description"
                    }
                    textarea {
                        class: "form-textarea",
                        value: "{description}",
                        placeholder: "Optional description",
                        oninput: move |evt| description.set(evt.value().clone())
                    }
                }
                
                // Tags
                div {
                    class: "form-field full-width",
                    label {
                        class: "form-label",
                        "Tags"
                    }
                    div {
                        class: "tag-input-container",
                        input {
                            class: "form-input",
                            r#type: "text",
                            value: "{tag_input}",
                            placeholder: "Add a tag (no spaces, use hyphens)",
                            oninput: move |evt| tag_input.set(evt.value().clone())
                        }
                        button {
                            class: "button-secondary",
                            onclick: add_tag,
                            "Add"
                        }
                    }
                    
                    // Display current tags
                    div {
                        class: "tag-list",
                        
                        for (idx, tag) in tags().iter().enumerate() {
                            div {
                                class: "tag-item",
                                span { "{tag}" }
                                button {
                                    class: "tag-remove",
                                    onclick: move |_| remove_tag(idx),
                                    "×"
                                }
                            }
                        }
                    }
                }
            }
            
            // Form actions
            div {
                class: "form-actions",
                
                button {
                    class: "button-secondary",
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
                
                button {
                    class: "button-primary",
                    onclick: save,
                    if entry.is_some() { "Update" } else { "Save" }
                }
            }
        }
    }
}
