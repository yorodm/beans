use crate::state::AppState;
use beans_lib::{
    models::{Currency, EntryType, LedgerEntryBuilder},
    BeansResult,
};
use chrono::{Local, NaiveDate};
use freya::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

/// Properties for the AddEntry component
#[derive(Props, Clone, PartialEq)]
pub struct AddEntryProps {
    /// Application state
    pub state: AppState,
}

/// AddEntry component for adding a new entry to the ledger
pub fn AddEntry(props: AddEntryProps) -> Element {
    let AddEntryProps { state } = props;
    
    // Form state
    let date = use_signal(|| Local::now().date_naive());
    let name = use_signal(|| String::new());
    let amount = use_signal(|| String::new());
    let currency = use_signal(|| String::from("USD"));
    let entry_type = use_signal(|| EntryType::Expense);
    let description = use_signal(|| String::new());
    let tags = use_signal(|| String::new());
    
    // Success message
    let success_message = use_signal(|| Option::<String>::None);
    
    // Format a date for display
    let format_date = |date: NaiveDate| date.format("%Y-%m-%d").to_string();
    
    // Add the entry to the ledger
    let add_entry = move || {
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
        
        // Build the entry
        let entry_result: BeansResult<_> = (|| {
            let mut builder = LedgerEntryBuilder::new()
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
            
            let entry = builder.build()?;
            
            // Add the entry to the ledger
            if let Some(manager) = state.ledger_manager.read().as_ref() {
                manager.add_entry(&entry)?;
            } else {
                return Err(beans_lib::BeansError::LedgerNotOpen);
            }
            
            Ok(())
        })();
        
        match entry_result {
            Ok(_) => {
                // Clear form
                name.set(String::new());
                amount.set(String::new());
                description.set(String::new());
                tags.set(String::new());
                
                // Show success message
                success_message.set(Some("Entry added successfully".to_string()));
                
                // Clear success message after 3 seconds
                let success = success_message.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    success.set(None);
                });
            }
            Err(e) => {
                state.set_error(format!("Failed to add entry: {}", e));
            }
        }
    };
    
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
                "Add New Entry"
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
                            value: "{format_date(*date.read())}",
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
                    main_align: "end",
                    margin_top: "10px",
                    
                    rect {
                        padding: "10px 20px",
                        background: "rgb(46, 139, 87)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| add_entry(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Add Entry"
                        }
                    }
                }
            }
        }
    }
}

