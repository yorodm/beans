use crate::state::{AppState, EntryFilter};
use beans_lib::{
    models::{Currency, EntryType},
    BeansResult,
};
use chrono::NaiveDate;
use freya::prelude::*;
use rfd::FileDialog;
use std::path::PathBuf;

/// Properties for the Export component
#[derive(Props, Clone, PartialEq)]
pub struct ExportProps {
    /// Application state
    pub state: AppState,
}

/// Export component for exporting the ledger
pub fn Export(props: ExportProps) -> Element {
    let ExportProps { state } = props;
    let filter = state.filter.read().clone();
    
    // Export format
    let export_format = use_signal(|| String::from("json"));
    
    // Filter state
    let use_filter = use_signal(|| false);
    let start_date = use_signal(|| filter.start_date);
    let end_date = use_signal(|| filter.end_date);
    let tag_filter = use_signal(|| filter.tags.join(", "));
    let currency_filter = use_signal(|| filter.currency.map(|c| c.to_string()).unwrap_or_default());
    let entry_type_filter = use_signal(|| filter.entry_type);
    
    // Success message
    let success_message = use_signal(|| Option::<String>::None);
    
    // Format a date for display
    let format_date = |date: Option<NaiveDate>| {
        date.map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "".to_string())
    };
    
    // Export the ledger
    let export_ledger = move || {
        if let Some(manager) = state.ledger_manager.read().as_ref() {
            // Get the export format
            let format = export_format.read().clone();
            
            // Create a filter if needed
            let mut db_filter = beans_lib::database::EntryFilter::default();
            
            if *use_filter.read() {
                if let Some(start) = start_date.read().as_ref() {
                    db_filter.start_date = Some(*start);
                }
                
                if let Some(end) = end_date.read().as_ref() {
                    db_filter.end_date = Some(*end);
                }
                
                // Parse tags from the tag filter
                let tags: Vec<String> = tag_filter
                    .read()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                    
                if !tags.is_empty() {
                    db_filter.tags = Some(tags);
                }
                
                // Parse currency
                if !currency_filter.read().is_empty() {
                    match Currency::from_str(&currency_filter.read()) {
                        Ok(c) => db_filter.currency = Some(c),
                        Err(_) => {
                            state.set_error("Invalid currency".to_string());
                            return;
                        }
                    }
                }
                
                // Entry type
                db_filter.entry_type = *entry_type_filter.read();
            }
            
            // Get the export file path
            let file_path = FileDialog::new()
                .add_filter("JSON", &["json"])
                .add_filter("CSV", &["csv"])
                .add_filter("XML", &["xml"])
                .set_directory("/")
                .set_file_name(&format!("ledger_export.{}", format.to_lowercase()))
                .save_file();
                
            if let Some(path) = file_path {
                // Export the ledger
                let result: BeansResult<()> = match format.as_str() {
                    "json" => manager.export_json(&path, Some(db_filter)),
                    "csv" => manager.export_csv(&path, Some(db_filter)),
                    "xml" => manager.export_xml(&path, Some(db_filter)),
                    _ => {
                        state.set_error(format!("Unsupported export format: {}", format));
                        return;
                    }
                };
                
                match result {
                    Ok(_) => {
                        // Show success message
                        success_message.set(Some(format!("Ledger exported successfully to {}", path.display())));
                        
                        // Clear success message after 5 seconds
                        let success = success_message.clone();
                        std::thread::spawn(move || {
                            std::thread::sleep(std::time::Duration::from_secs(5));
                            success.set(None);
                        });
                    }
                    Err(e) => {
                        state.set_error(format!("Failed to export ledger: {}", e));
                    }
                }
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
                "Export Ledger"
            }
            
            // Export options
            rect {
                width: "100%",
                max_width: "800px",
                padding: "20px",
                background: "rgb(240, 240, 240)",
                border_radius: "5px",
                direction: "vertical",
                gap: "20px",
                
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
                
                // Export format
                rect {
                    width: "100%",
                    direction: "vertical",
                    gap: "10px",
                    
                    label {
                        font_size: "16px",
                        font_weight: "bold",
                        color: "rgb(50, 50, 50)",
                        "Export Format"
                    }
                    
                    rect {
                        width: "100%",
                        direction: "horizontal",
                        gap: "20px",
                        
                        // JSON
                        rect {
                            direction: "horizontal",
                            gap: "5px",
                            cursor: "pointer",
                            onclick: move |_| export_format.set("json".to_string()),
                            
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
                                    background: if *export_format.read() == "json" {
                                        "rgb(70, 130, 180)"
                                    } else {
                                        "transparent"
                                    },
                                }
                            }
                            
                            label {
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                "JSON"
                            }
                        }
                        
                        // CSV
                        rect {
                            direction: "horizontal",
                            gap: "5px",
                            cursor: "pointer",
                            onclick: move |_| export_format.set("csv".to_string()),
                            
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
                                    background: if *export_format.read() == "csv" {
                                        "rgb(70, 130, 180)"
                                    } else {
                                        "transparent"
                                    },
                                }
                            }
                            
                            label {
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                "CSV"
                            }
                        }
                        
                        // XML
                        rect {
                            direction: "horizontal",
                            gap: "5px",
                            cursor: "pointer",
                            onclick: move |_| export_format.set("xml".to_string()),
                            
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
                                    background: if *export_format.read() == "xml" {
                                        "rgb(70, 130, 180)"
                                    } else {
                                        "transparent"
                                    },
                                }
                            }
                            
                            label {
                                font_size: "14px",
                                color: "rgb(50, 50, 50)",
                                "XML"
                            }
                        }
                    }
                }
                
                // Filter options
                rect {
                    width: "100%",
                    direction: "vertical",
                    gap: "15px",
                    
                    // Filter toggle
                    rect {
                        width: "100%",
                        direction: "horizontal",
                        gap: "10px",
                        
                        rect {
                            width: "24px",
                            height: "24px",
                            border: "2px solid rgb(200, 200, 200)",
                            border_radius: "4px",
                            main_align: "center",
                            cross_align: "center",
                            cursor: "pointer",
                            onclick: move |_| use_filter.set(!*use_filter.read()),
                            
                            {
                                if *use_filter.read() {
                                    rsx! {
                                        rect {
                                            width: "14px",
                                            height: "14px",
                                            background: "rgb(70, 130, 180)",
                                            border_radius: "2px",
                                        }
                                    }
                                } else {
                                    rsx! { fragment {} }
                                }
                            }
                        }
                        
                        label {
                            font_size: "16px",
                            font_weight: "bold",
                            color: "rgb(50, 50, 50)",
                            cursor: "pointer",
                            onclick: move |_| use_filter.set(!*use_filter.read()),
                            "Apply Filters"
                        }
                    }
                    
                    // Filter controls (only shown if use_filter is true)
                    {
                        if *use_filter.read() {
                            rsx! {
                                rect {
                                    width: "100%",
                                    direction: "vertical",
                                    gap: "15px",
                                    padding: "10px 0 0 20px",
                                    
                                    // Date range
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
                                            "Date Range:"
                                        }
                                        
                                        // Start date
                                        rect {
                                            width: "150px",
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
                                                placeholder: "Start Date",
                                                value: "{format_date(start_date.read().clone())}",
                                                oninput: move |e| {
                                                    if e.value.is_empty() {
                                                        start_date.set(None);
                                                    } else {
                                                        match NaiveDate::parse_from_str(&e.value, "%Y-%m-%d") {
                                                            Ok(date) => start_date.set(Some(date)),
                                                            Err(_) => {} // Ignore invalid dates
                                                        }
                                                    }
                                                },
                                            }
                                        }
                                        
                                        label {
                                            font_size: "14px",
                                            color: "rgb(50, 50, 50)",
                                            main_align: "center",
                                            cross_align: "center",
                                            "to"
                                        }
                                        
                                        // End date
                                        rect {
                                            width: "150px",
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
                                                placeholder: "End Date",
                                                value: "{format_date(end_date.read().clone())}",
                                                oninput: move |e| {
                                                    if e.value.is_empty() {
                                                        end_date.set(None);
                                                    } else {
                                                        match NaiveDate::parse_from_str(&e.value, "%Y-%m-%d") {
                                                            Ok(date) => end_date.set(Some(date)),
                                                            Err(_) => {} // Ignore invalid dates
                                                        }
                                                    }
                                                },
                                            }
                                        }
                                    }
                                    
                                    // Tags
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
                                                value: "{tag_filter}",
                                                oninput: move |e| tag_filter.set(e.value.clone()),
                                            }
                                        }
                                    }
                                    
                                    // Currency
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
                                            "Currency:"
                                        }
                                        
                                        rect {
                                            width: "150px",
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
                                                placeholder: "Currency (e.g. USD)",
                                                value: "{currency_filter}",
                                                oninput: move |e| currency_filter.set(e.value.clone()),
                                            }
                                        }
                                    }
                                    
                                    // Entry Type
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
                                            "Type:"
                                        }
                                        
                                        rect {
                                            direction: "horizontal",
                                            gap: "20px",
                                            
                                            // All
                                            rect {
                                                direction: "horizontal",
                                                gap: "5px",
                                                cursor: "pointer",
                                                onclick: move |_| entry_type_filter.set(None),
                                                
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
                                                        background: if entry_type_filter.read().is_none() {
                                                            "rgb(70, 130, 180)"
                                                        } else {
                                                            "transparent"
                                                        },
                                                    }
                                                }
                                                
                                                label {
                                                    font_size: "14px",
                                                    color: "rgb(50, 50, 50)",
                                                    "All"
                                                }
                                            }
                                            
                                            // Income
                                            rect {
                                                direction: "horizontal",
                                                gap: "5px",
                                                cursor: "pointer",
                                                onclick: move |_| entry_type_filter.set(Some(EntryType::Income)),
                                                
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
                                                        background: if let Some(EntryType::Income) = *entry_type_filter.read() {
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
                                                onclick: move |_| entry_type_filter.set(Some(EntryType::Expense)),
                                                
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
                                                        background: if let Some(EntryType::Expense) = *entry_type_filter.read() {
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
                                }
                            }
                        } else {
                            rsx! { fragment {} }
                        }
                    }
                }
                
                // Export button
                rect {
                    width: "100%",
                    main_align: "end",
                    margin_top: "20px",
                    
                    rect {
                        padding: "10px 20px",
                        background: "rgb(70, 130, 180)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| export_ledger(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Export Ledger"
                        }
                    }
                }
            }
        }
    }
}

