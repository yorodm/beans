use crate::state::AppState;
use beans_lib::{
    database::EntryFilter as DbEntryFilter,
    models::{Currency, EntryType, LedgerEntry, LedgerEntryBuilder},
    BeansResult,
};
use chrono::{Local, NaiveDate};
use freya::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

/// Properties for the EditEntry component
#[derive(Props, Clone, PartialEq)]
pub struct EditEntryProps {
    /// Application state
    pub state: AppState,
}

/// EditEntry component for editing an existing entry
pub fn EditEntry(props: EditEntryProps) -> Element {
    let EditEntryProps { state } = props;
    
    // Selected entry
    let selected_entry = state.selected_entry.read().clone();
    
    // Filter state for entry selection
    let filter_date = use_signal(|| Option::<NaiveDate>::None);
    let filter_tags = use_signal(|| String::new());
    
    // List of entries matching the filter
    let entries = use_signal(|| Vec::<LedgerEntry>::new());
    
    // Format a date for display
    let format_date = |date: Option<NaiveDate>| {
        date.map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "".to_string())
    };
    
    // Apply the filter to find entries
    let apply_filter = move || {
        if let Some(manager) = state.ledger_manager.read().as_ref() {
            // Create a filter
            let mut db_filter = DbEntryFilter::default();
            
            if let Some(date) = *filter_date.read() {
                db_filter.start_date = Some(date);
                db_filter.end_date = Some(date);
            }
            
            // Parse tags from the filter
            let tags: Vec<String> = filter_tags
                .read()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
                
            if !tags.is_empty() {
                db_filter.tags = Some(tags);
            }
            
            // Get entries matching the filter
            match manager.get_entries(db_filter) {
                Ok(filtered_entries) => {
                    entries.set(filtered_entries);
                }
                Err(e) => {
                    state.set_error(format!("Failed to get entries: {}", e));
                }
            }
        }
    };
    
    // Select an entry for editing
    let select_entry = move |entry: LedgerEntry| {
        state.select_entry(entry);
    };
    
    // If an entry is selected, show the edit form
    if let Some(entry) = selected_entry {
        // Form state
        let id = entry.id;
        let date = use_signal(|| entry.date);
        let name = use_signal(|| entry.name.clone());
        let amount = use_signal(|| entry.amount.to_string());
        let currency = use_signal(|| entry.currency.to_string());
        let entry_type = use_signal(|| entry.entry_type);
        let description = use_signal(|| entry.description.clone().unwrap_or_default());
        let tags = use_signal(|| {
            entry.tags
                .iter()
                .map(|t| t.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        });
        
        // Success message
        let success_message = use_signal(|| Option::<String>::None);
        
        // Update the entry
        let update_entry = move || {
            // Validate form
            if name.read().is_empty() {
                state.set_error("Name is required".to_string());
                return;
            }
            
            if amount.read().is_empty() {
                state.set_error("Amount is required".to_string());
                return;
            }
            
            // Parse amount
            let amount_decimal = match Decimal::from_str(&amount.read()) {
                Ok(d) => d,
                Err(_) => {
                    state.set_error("Invalid amount".to_string());
                    return;
                }
            };
            
            // Parse currency
            let currency_value = match Currency::from_str(&currency.read()) {
                Ok(c) => c,
                Err(_) => {
                    state.set_error("Invalid currency".to_string());
                    return;
                }
            };
            
            // Parse tags
            let tags_vec: Vec<String> = tags
                .read()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            // Build the updated entry
            let entry_result: BeansResult<_> = (|| {
                let mut builder = LedgerEntryBuilder::new()
                    .with_id(id)
                    .with_date(*date.read())
                    .with_name(name.read().clone())
                    .with_amount(amount_decimal)
                    .with_currency(currency_value)
                    .with_entry_type(*entry_type.read());
                    
                if !description.read().is_empty() {
                    builder = builder.with_description(description.read().clone());
                }
                
                for tag in &tags_vec {
                    builder = builder.with_tag(tag)?;
                }
                
                let updated_entry = builder.build()?;
                
                // Update the entry in the ledger
                if let Some(manager) = state.ledger_manager.read().as_ref() {
                    manager.update_entry(&updated_entry)?;
                } else {
                    return Err(beans_lib::BeansError::LedgerNotOpen);
                }
                
                Ok(())
            })();
            
            match entry_result {
                Ok(_) => {
                    // Show success message
                    success_message.set(Some("Entry updated successfully".to_string()));
                    
                    // Clear success message after 3 seconds
                    let success = success_message.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_secs(3));
                        success.set(None);
                    });
                }
                Err(e) => {
                    state.set_error(format!("Failed to update entry: {}", e));
                }
            }
        };
        
        // Cancel editing
        let cancel_edit = move || {
            state.clear_selected_entry();
        };
        
        // Edit form
        rsx! {
            rect {
                width: "100%",
                height: "calc(100% - 80px)",
                padding: "20px",
                direction: "vertical",
                gap: "20px",
                
                // Title
                rect {
                    width: "100%",
                    direction: "horizontal",
                    main_align: "space-between",
                    
                    label {
                        font_size: "24px",
                        font_weight: "bold",
                        color: "rgb(50, 50, 50)",
                        "Edit Entry"
                    }
                    
                    // Back button
                    rect {
                        padding: "8px 15px",
                        background: "rgb(100, 100, 100)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| cancel_edit(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Back to List"
                        }
                    }
                }
                
                // Form
                rect {
                    width: "100%",
                    max_width: "600px",
                    padding: "20px",
                    background: "rgb(240, 240, 240)",
                    border_radius: "5px",
                    direction: "vertical",
                    gap: "15px",
                    
                    // Success message
                    {
                        if let Some(message) = success_message.read().as_ref() {
                            rsx! {
                                rect {
                                    width: "100%",
                                    padding: "10px",
                                    background: "rgb(46, 139, 87, 0.2)",
                                    border: "1px solid rgb(46, 139, 87)",
                                    border_radius: "4px",
                                    
                                    label {
                                        font_size: "14px",
                                        color: "rgb(46, 139, 87)",
                                        "{message}"
                                    }
                                }
                            }
                        } else {
                            rsx! { fragment {} }
                        }
                    }
                    
                    // Date
                    rect {
                        width: "100%",
                        direction: "vertical",
                        gap: "5px",
                        
                        label {
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            "Date"
                        }
                        
                        rect {
                            width: "100%",
                            height: "40px",
                            padding: "0 10px",
                            background: "white",
                            border: "1px solid rgb(200, 200, 200)",
                            border_radius: "4px",
                            
                            input {
                                width: "100%",
                                height: "100%",
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                value: "{format_date(Some(*date.read()))}",
                                oninput: move |e| {
                                    if let Ok(d) = NaiveDate::parse_from_str(&e.value, "%Y-%m-%d") {
                                        date.set(d);
                                    }
                                },
                            }
                        }
                    }
                    
                    // Name
                    rect {
                        width: "100%",
                        direction: "vertical",
                        gap: "5px",
                        
                        label {
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            "Name *"
                        }
                        
                        rect {
                            width: "100%",
                            height: "40px",
                            padding: "0 10px",
                            background: "white",
                            border: "1px solid rgb(200, 200, 200)",
                            border_radius: "4px",
                            
                            input {
                                width: "100%",
                                height: "100%",
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                placeholder: "Entry name",
                                value: "{name}",
                                oninput: move |e| name.set(e.value.clone()),
                            }
                        }
                    }
                    
                    // Amount and Currency
                    rect {
                        width: "100%",
                        direction: "horizontal",
                        gap: "10px",
                        
                        // Amount
                        rect {
                            width: "60%",
                            direction: "vertical",
                            gap: "5px",
                            
                            label {
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                "Amount *"
                            }
                            
                            rect {
                                width: "100%",
                                height: "40px",
                                padding: "0 10px",
                                background: "white",
                                border: "1px solid rgb(200, 200, 200)",
                                border_radius: "4px",
                                
                                input {
                                    width: "100%",
                                    height: "100%",
                                    font_size: "14px",
                                    color: "rgb(50, 50, 50)",
                                    placeholder: "0.00",
                                    value: "{amount}",
                                    oninput: move |e| amount.set(e.value.clone()),
                                }
                            }
                        }
                        
                        // Currency
                        rect {
                            width: "40%",
                            direction: "vertical",
                            gap: "5px",
                            
                            label {
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                "Currency"
                            }
                            
                            rect {
                                width: "100%",
                                height: "40px",
                                padding: "0 10px",
                                background: "white",
                                border: "1px solid rgb(200, 200, 200)",
                                border_radius: "4px",
                                
                                input {
                                    width: "100%",
                                    height: "100%",
                                    font_size: "14px",
                                    color: "rgb(50, 50, 50)",
                                    placeholder: "USD",
                                    value: "{currency}",
                                    oninput: move |e| currency.set(e.value.clone()),
                                }
                            }
                        }
                    }
                    
                    // Entry Type
                    rect {
                        width: "100%",
                        direction: "vertical",
                        gap: "5px",
                        
                        label {
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            "Entry Type"
                        }
                        
                        rect {
                            width: "100%",
                            direction: "horizontal",
                            gap: "20px",
                            
                            // Income
                            rect {
                                direction: "horizontal",
                                gap: "5px",
                                cursor: "pointer",
                                onclick: move |_| entry_type.set(EntryType::Income),
                                
                                rect {
                                    width: "20px",
                                    height: "20px",
                                    border: "2px solid rgb(200, 200, 200)",
                                    border_radius: "50%",
                                    main_align: "center",
                                    cross_align: "center",
                                    
                                    rect {
                                        width: "12px",
                                        height: "12px",
                                        border_radius: "50%",
                                        background: if *entry_type.read() == EntryType::Income {
                                            "rgb(70, 130, 180)"
                                        } else {
                                            "transparent"
                                        },
                                    }
                                }
                                
                                label {
                                    font_size: "14px",
                                    color: "rgb(50, 50, 50)",
                                    "Income"
                                }
                            }
                            
                            // Expense
                            rect {
                                direction: "horizontal",
                                gap: "5px",
                                cursor: "pointer",
                                onclick: move |_| entry_type.set(EntryType::Expense),
                                
                                rect {
                                    width: "20px",
                                    height: "20px",
                                    border: "2px solid rgb(200, 200, 200)",
                                    border_radius: "50%",
                                    main_align: "center",
                                    cross_align: "center",
                                    
                                    rect {
                                        width: "12px",
                                        height: "12px",
                                        border_radius: "50%",
                                        background: if *entry_type.read() == EntryType::Expense {
                                            "rgb(70, 130, 180)"
                                        } else {
                                            "transparent"
                                        },
                                    }
                                }
                                
                                label {
                                    font_size: "14px",
                                    color: "rgb(50, 50, 50)",
                                    "Expense"
                                }
                            }
                        }
                    }
                    
                    // Description
                    rect {
                        width: "100%",
                        direction: "vertical",
                        gap: "5px",
                        
                        label {
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            "Description"
                        }
                        
                        rect {
                            width: "100%",
                            height: "80px",
                            padding: "10px",
                            background: "white",
                            border: "1px solid rgb(200, 200, 200)",
                            border_radius: "4px",
                            
                            textarea {
                                width: "100%",
                                height: "100%",
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                placeholder: "Description (optional)",
                                value: "{description}",
                                oninput: move |e| description.set(e.value.clone()),
                            }
                        }
                    }
                    
                    // Tags
                    rect {
                        width: "100%",
                        direction: "vertical",
                        gap: "5px",
                        
                        label {
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            "Tags"
                        }
                        
                        rect {
                            width: "100%",
                            height: "40px",
                            padding: "0 10px",
                            background: "white",
                            border: "1px solid rgb(200, 200, 200)",
                            border_radius: "4px",
                            
                            input {
                                width: "100%",
                                height: "100%",
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                placeholder: "Tags (comma separated)",
                                value: "{tags}",
                                oninput: move |e| tags.set(e.value.clone()),
                            }
                        }
                    }
                    
                    // Submit button
                    rect {
                        width: "100%",
                        direction: "horizontal",
                        main_align: "end",
                        gap: "10px",
                        margin_top: "10px",
                        
                        // Cancel button
                        rect {
                            padding: "10px 20px",
                            background: "rgb(100, 100, 100)",
                            border_radius: "4px",
                            cursor: "pointer",
                            onclick: move |_| cancel_edit(),
                            
                            label {
                                color: "white",
                                font_size: "14px",
                                "Cancel"
                            }
                        }
                        
                        // Save button
                        rect {
                            padding: "10px 20px",
                            background: "rgb(70, 130, 180)",
                            border_radius: "4px",
                            cursor: "pointer",
                            onclick: move |_| update_entry(),
                            
                            label {
                                color: "white",
                                font_size: "14px",
                                "Save Changes"
                            }
                        }
                    }
                }
            }
        }
    } else {
        // Entry selection view
        rsx! {
            rect {
                width: "100%",
                height: "calc(100% - 80px)",
                padding: "20px",
                direction: "vertical",
                gap: "20px",
                
                // Title
                label {
                    font_size: "24px",
                    font_weight: "bold",
                    color: "rgb(50, 50, 50)",
                    "Edit Entry"
                }
                
                // Filter controls
                rect {
                    width: "100%",
                    padding: "15px",
                    background: "rgb(240, 240, 240)",
                    border_radius: "5px",
                    direction: "vertical",
                    gap: "15px",
                    
                    label {
                        font_size: "16px",
                        font_weight: "bold",
                        color: "rgb(50, 50, 50)",
                        "Filter Entries"
                    }
                    
                    // Date filter
                    rect {
                        width: "100%",
                        direction: "horizontal",
                        gap: "10px",
                        
                        label {
                            width: "80px",
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            main_align: "center",
                            cross_align: "center",
                            "Date:"
                        }
                        
                        rect {
                            width: "200px",
                            height: "30px",
                            padding: "0 10px",
                            background: "white",
                            border: "1px solid rgb(200, 200, 200)",
                            border_radius: "4px",
                            
                            input {
                                width: "100%",
                                height: "100%",
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                placeholder: "Filter by date",
                                value: "{format_date(filter_date.read().clone())}",
                                oninput: move |e| {
                                    if e.value.is_empty() {
                                        filter_date.set(None);
                                    } else {
                                        match NaiveDate::parse_from_str(&e.value, "%Y-%m-%d") {
                                            Ok(date) => filter_date.set(Some(date)),
                                            Err(_) => {} // Ignore invalid dates
                                        }
                                    }
                                },
                            }
                        }
                    }
                    
                    // Tags filter
                    rect {
                        width: "100%",
                        direction: "horizontal",
                        gap: "10px",
                        
                        label {
                            width: "80px",
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            main_align: "center",
                            cross_align: "center",
                            "Tags:"
                        }
                        
                        rect {
                            width: "calc(100% - 90px)",
                            height: "30px",
                            padding: "0 10px",
                            background: "white",
                            border: "1px solid rgb(200, 200, 200)",
                            border_radius: "4px",
                            
                            input {
                                width: "100%",
                                height: "100%",
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                placeholder: "Tags (comma separated)",
                                value: "{filter_tags}",
                                oninput: move |e| filter_tags.set(e.value.clone()),
                            }
                        }
                    }
                    
                    // Apply button
                    rect {
                        width: "100%",
                        main_align: "end",
                        
                        rect {
                            padding: "8px 15px",
                            background: "rgb(70, 130, 180)",
                            border_radius: "4px",
                            cursor: "pointer",
                            onclick: move |_| apply_filter(),
                            
                            label {
                                color: "white",
                                font_size: "14px",
                                "Find Entries"
                            }
                        }
                    }
                }
                
                // Entries list
                rect {
                    width: "100%",
                    height: "calc(100% - 200px)",
                    min_height: "300px",
                    padding: "15px",
                    background: "rgb(240, 240, 240)",
                    border_radius: "5px",
                    direction: "vertical",
                    gap: "15px",
                    overflow: "auto",
                    
                    label {
                        font_size: "16px",
                        font_weight: "bold",
                        color: "rgb(50, 50, 50)",
                        "Entries"
                    }
                    
                    // List of entries
                    rect {
                        width: "100%",
                        direction: "vertical",
                        gap: "10px",
                        
                        // Show message if no entries
                        {
                            if entries.read().is_empty() {
                                rsx! {
                                    rect {
                                        width: "100%",
                                        padding: "20px",
                                        main_align: "center",
                                        
                                        label {
                                            font_size: "14px",
                                            color: "rgb(100, 100, 100)",
                                            "No entries found. Apply a filter to see entries."
                                        }
                                    }
                                }
                            } else {
                                rsx! { fragment {} }
                            }
                        }
                        
                        // Entry items
                        {
                            entries.read().iter().map(|entry| {
                                let entry_clone = entry.clone();
                                let entry_type_color = if entry.entry_type == EntryType::Income {
                                    "rgb(46, 139, 87)"
                                } else {
                                    "rgb(178, 34, 34)"
                                };
                                
                                let entry_tags = entry.tags
                                    .iter()
                                    .map(|t| t.name.clone())
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                
                                rsx! {
                                    rect {
                                        width: "100%",
                                        padding: "15px",
                                        background: "white",
                                        border_radius: "4px",
                                        direction: "vertical",
                                        gap: "10px",
                                        
                                        // Entry header
                                        rect {
                                            width: "100%",
                                            direction: "horizontal",
                                            main_align: "space-between",
                                            
                                            // Name and date
                                            rect {
                                                direction: "vertical",
                                                gap: "5px",
                                                
                                                label {
                                                    font_size: "16px",
                                                    font_weight: "bold",
                                                    color: "rgb(50, 50, 50)",
                                                    "{entry.name}"
                                                }
                                                
                                                label {
                                                    font_size: "12px",
                                                    color: "rgb(100, 100, 100)",
                                                    "{entry.date.format(\"%Y-%m-%d\")}"
                                                }
                                            }
                                            
                                            // Amount and type
                                            rect {
                                                direction: "vertical",
                                                gap: "5px",
                                                cross_align: "end",
                                                
                                                label {
                                                    font_size: "16px",
                                                    font_weight: "bold",
                                                    color: "{entry_type_color}",
                                                    "{entry.amount} {entry.currency}"
                                                }
                                                
                                                label {
                                                    font_size: "12px",
                                                    color: "{entry_type_color}",
                                                    "{format!(\"{:?}\", entry.entry_type)}"
                                                }
                                            }
                                        }
                                        
                                        // Description if available
                                        {
                                            if let Some(desc) = &entry.description {
                                                if !desc.is_empty() {
                                                    rsx! {
                                                        rect {
                                                            width: "100%",
                                                            padding: "5px 0",
                                                            
                                                            label {
                                                                font_size: "14px",
                                                                color: "rgb(80, 80, 80)",
                                                                "{desc}"
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    rsx! { fragment {} }
                                                }
                                            } else {
                                                rsx! { fragment {} }
                                            }
                                        }
                                        
                                        // Tags if available
                                        {
                                            if !entry_tags.is_empty() {
                                                rsx! {
                                                    rect {
                                                        width: "100%",
                                                        
                                                        label {
                                                            font_size: "12px",
                                                            color: "rgb(100, 100, 100)",
                                                            "Tags: {entry_tags}"
                                                        }
                                                    }
                                                }
                                            } else {
                                                rsx! { fragment {} }
                                            }
                                        }
                                        
                                        // Edit button
                                        rect {
                                            width: "100%",
                                            main_align: "end",
                                            
                                            rect {
                                                padding: "8px 15px",
                                                background: "rgb(70, 130, 180)",
                                                border_radius: "4px",
                                                cursor: "pointer",
                                                onclick: move |_| select_entry(entry_clone.clone()),
                                                
                                                label {
                                                    color: "white",
                                                    font_size: "14px",
                                                    "Edit"
                                                }
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

