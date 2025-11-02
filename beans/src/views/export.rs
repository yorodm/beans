//! Export view for exporting ledger data to different formats

use crate::components::filter_panel::FilterPanel;
use crate::state::{AppState, View};
use beans_lib::prelude::*;
use beans_lib::reporting::{ExportFormat, ReportGenerator};
use dioxus::prelude::*;
use std::path::PathBuf;
use chrono::Utc;
use std::sync::Arc;
use std::collections::HashMap;

/// Export View
/// 
/// This view provides functionality to export ledger data with:
/// - Filter configuration (reuse FilterPanel)
/// - Format selection (JSON/CSV radio buttons)
/// - Preview and save area
#[component]
pub fn ExportView() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    
    // Local state
    let format = use_signal(|| "json".to_string());
    let export_path = use_signal(|| {
        app_state
            .read()
            .ledger_path
            .as_ref()
            .map(|p| {
                let mut path = p.clone();
                path.set_extension(format());
                path.to_string_lossy().to_string()
            })
            .unwrap_or_default()
    });
    let preview_content = use_signal(String::new);
    let has_preview = use_signal(|| false);
    
    // Handle filter apply
    let on_filter_apply = move |_| {
        // Filtering is handled by the FilterPanel component
        // We just need to refresh the view
        has_preview.set(false);
    };
    
    // Handle format change
    let on_format_change = move |new_format: String| {
        format.set(new_format.clone());
        
        // Update export path extension
        if let Some(path) = &app_state.read().ledger_path {
            let mut new_path = path.clone();
            new_path.set_extension(new_format);
            export_path.set(new_path.to_string_lossy().to_string());
        }
        
        // Clear preview when format changes
        has_preview.set(false);
    };
    
    // Generate report
    let generate_report = move |_| {
        // Clone all signals and values we need to use in the async block
        let app_state = app_state.clone();
        let format = format.clone();
        let preview_content = preview_content.clone();
        let has_preview = has_preview.clone();
        
        // Spawn the async operation
        spawn(async move {
            // We need to extract all data we need while holding the read lock
            // Then release the lock before doing async operations
            
            // First, check if we have a ledger manager and extract what we need
            let (start_date, end_date, tag_list, format_value, ledger_entries) = {
                let state = app_state.read();
                
                if state.ledger_manager.is_none() {
                    drop(state);
                    app_state.write().set_error("No ledger is open".to_string());
                    return;
                }
                
                let manager = state.ledger_manager.as_ref().unwrap();
                
                // Get start date
                let start = state.filter.date_range.start.unwrap_or_else(|| {
                    // Default to 30 days ago if no start date
                    Utc::now() - chrono::Duration::days(30)
                });
                
                // Get end date
                let end = state.filter.date_range.end.unwrap_or_else(Utc::now);
                
                // Get tags
                let tags: Vec<String> = state.filter.tags.iter()
                    .map(|t| t.clone())
                    .collect();
                
                // Get format
                let fmt = format();
                
                // Create a filter to get entries
                let mut filter = EntryFilter::new();
                filter.start_date = Some(start);
                filter.end_date = Some(end);
                for tag in &state.filter.tags {
                    filter.tags.push(tag.clone());
                }
                
                // Get entries (we need to do this while we have the manager reference)
                let entries = match manager.list_entries(&filter) {
                    Ok(e) => e,
                    Err(err) => {
                        drop(state);
                        app_state.write().set_error(format!("Failed to list entries: {}", err));
                        return;
                    }
                };
                
                (start, end, tags, fmt, entries)
            };
            
            // Now we can work with the extracted data without holding the read lock
            
            // Convert format string to ExportFormat
            let export_format = match format_value.as_str() {
                "json" => ExportFormat::Json,
                "csv" => ExportFormat::Csv,
                _ => {
                    app_state.write().set_error("Unsupported export format".to_string());
                    return;
                }
            };
            
            // Since we can't use ReportGenerator directly (it needs a reference to LedgerManager),
            // we'll manually format the data similar to what ReportGenerator would do
            
            // For simplicity, we'll create a basic report format
            let content = match export_format {
                ExportFormat::Json => {
                    // Create a simple JSON report
                    let mut income_total = 0.0;
                    let mut expense_total = 0.0;
                    
                    // Group entries by tag
                    let mut entries_by_tag: HashMap<String, Vec<&LedgerEntry>> = HashMap::new();
                    
                    for entry in &ledger_entries {
                        // Update totals
                        match entry.entry_type() {
                            EntryType::Income => income_total += entry.amount().to_f64().unwrap_or(0.0),
                            EntryType::Expense => expense_total += entry.amount().to_f64().unwrap_or(0.0),
                        }
                        
                        // Group by tags
                        let tags = if entry.tags().is_empty() {
                            vec!["Untagged".to_string()]
                        } else {
                            entry.tags().iter().map(|t| t.name().to_string()).collect()
                        };
                        
                        for tag in tags {
                            entries_by_tag.entry(tag).or_insert_with(Vec::new).push(entry);
                        }
                    }
                    
                    // Create a simple JSON report
                    let mut json = String::from("{\n");
                    json.push_str("  \"summary\": {\n");
                    json.push_str(&format!("    \"income\": {},\n", income_total));
                    json.push_str(&format!("    \"expenses\": {},\n", expense_total));
                    json.push_str(&format!("    \"net\": {}\n", income_total - expense_total));
                    json.push_str("  },\n");
                    
                    json.push_str("  \"by_tag\": {\n");
                    
                    let mut tags: Vec<String> = entries_by_tag.keys().cloned().collect();
                    tags.sort();
                    
                    for (i, tag) in tags.iter().enumerate() {
                        let entries = entries_by_tag.get(tag).unwrap();
                        
                        let tag_income: f64 = entries.iter()
                            .filter(|e| e.entry_type() == EntryType::Income)
                            .map(|e| e.amount().to_f64().unwrap_or(0.0))
                            .sum();
                            
                        let tag_expenses: f64 = entries.iter()
                            .filter(|e| e.entry_type() == EntryType::Expense)
                            .map(|e| e.amount().to_f64().unwrap_or(0.0))
                            .sum();
                        
                        json.push_str(&format!("    \"{}\": {{\n", tag));
                        json.push_str(&format!("      \"income\": {},\n", tag_income));
                        json.push_str(&format!("      \"expenses\": {},\n", tag_expenses));
                        json.push_str(&format!("      \"net\": {}\n", tag_income - tag_expenses));
                        
                        if i < tags.len() - 1 {
                            json.push_str("    },\n");
                        } else {
                            json.push_str("    }\n");
                        }
                    }
                    
                    json.push_str("  }\n");
                    json.push_str("}");
                    
                    json
                },
                ExportFormat::Csv => {
                    // Create a simple CSV report
                    let mut csv = String::from("Tag,Income,Expenses,Net\n");
                    
                    // Group entries by tag
                    let mut entries_by_tag: HashMap<String, Vec<&LedgerEntry>> = HashMap::new();
                    
                    let mut income_total = 0.0;
                    let mut expense_total = 0.0;
                    
                    for entry in &ledger_entries {
                        // Update totals
                        match entry.entry_type() {
                            EntryType::Income => income_total += entry.amount().to_f64().unwrap_or(0.0),
                            EntryType::Expense => expense_total += entry.amount().to_f64().unwrap_or(0.0),
                        }
                        
                        // Group by tags
                        let tags = if entry.tags().is_empty() {
                            vec!["Untagged".to_string()]
                        } else {
                            entry.tags().iter().map(|t| t.name().to_string()).collect()
                        };
                        
                        for tag in tags {
                            entries_by_tag.entry(tag).or_insert_with(Vec::new).push(entry);
                        }
                    }
                    
                    // Add rows for each tag
                    let mut tags: Vec<String> = entries_by_tag.keys().cloned().collect();
                    tags.sort();
                    
                    for tag in tags {
                        let entries = entries_by_tag.get(&tag).unwrap();
                        
                        let tag_income: f64 = entries.iter()
                            .filter(|e| e.entry_type() == EntryType::Income)
                            .map(|e| e.amount().to_f64().unwrap_or(0.0))
                            .sum();
                            
                        let tag_expenses: f64 = entries.iter()
                            .filter(|e| e.entry_type() == EntryType::Expense)
                            .map(|e| e.amount().to_f64().unwrap_or(0.0))
                            .sum();
                        
                        csv.push_str(&format!("{},{},{},{}\n", 
                            tag, tag_income, tag_expenses, tag_income - tag_expenses));
                    }
                    
                    // Add summary
                    csv.push_str("\nSummary\n");
                    csv.push_str(&format!("Total Income,{}\n", income_total));
                    csv.push_str(&format!("Total Expenses,{}\n", expense_total));
                    csv.push_str(&format!("Net,{}\n", income_total - expense_total));
                    
                    csv
                }
            };
            
            // Update UI with the results
            preview_content.set(content);
            has_preview.set(true);
            app_state.write().set_success("Report generated successfully".to_string());
        });
    };
    
    // Save to file
    let save_to_file = move |_| {
        if !has_preview() {
            app_state.write().set_error("Generate a report first".to_string());
            return;
        }
        
        if export_path().trim().is_empty() {
            app_state.write().set_error("Export path is required".to_string());
            return;
        }
        
        let path = PathBuf::from(export_path());
        
        // Save the file
        if let Err(e) = std::fs::write(&path, preview_content()) {
            app_state.write().set_error(format!("Failed to save report: {}", e));
        } else {
            app_state.write().set_success(format!("Report saved to {}", path.display()));
        }
    };
    
    // Back to overview
    let back_to_overview = move |_| {
        app_state.write().set_view(View::Overview);
    };
    
    rsx! {
        div {
            class: "view export-view",
            
            // Header
            div {
                class: "view-header",
                h1 { "Export Ledger" }
                
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
            
            // Main content
            div {
                class: "export-content",
                
                // Left sidebar with filters
                div {
                    class: "export-sidebar",
                    
                    // Filter panel
                    div {
                        class: "filter-container",
                        h3 { "Filter Entries" }
                        FilterPanel {
                            on_apply: on_filter_apply
                        }
                    }
                    
                    // Format selection
                    div {
                        class: "format-container",
                        h3 { "Export Format" }
                        
                        div {
                            class: "radio-group",
                            
                            label {
                                class: "radio-label",
                                input {
                                    r#type: "radio",
                                    name: "export-format",
                                    value: "json",
                                    checked: format() == "json",
                                    oninput: move |_| on_format_change("json".to_string())
                                }
                                "JSON"
                            }
                            
                            label {
                                class: "radio-label",
                                input {
                                    r#type: "radio",
                                    name: "export-format",
                                    value: "csv",
                                    checked: format() == "csv",
                                    oninput: move |_| on_format_change("csv".to_string())
                                }
                                "CSV"
                            }
                        }
                        
                        div {
                            class: "format-help",
                            p {
                                strong { "JSON: " }
                                "Full data with all fields, best for backup or import to other systems."
                            }
                            p {
                                strong { "CSV: " }
                                "Simplified format, best for spreadsheet analysis."
                            }
                        }
                        
                        button {
                            class: "button-primary",
                            onclick: generate_report,
                            "Generate Report"
                        }
                    }
                }
                
                // Main area with preview and save
                div {
                    class: "export-main",
                    
                    // Preview area
                    div {
                        class: "preview-container",
                        h3 { "Preview" }
                        
                        if has_preview() {
                            div {
                                class: "preview-content",
                                pre {
                                    code {
                                        "{preview_content}"
                                    }
                                }
                            }
                            
                            // Save options
                            div {
                                class: "save-container",
                                
                                div {
                                    class: "form-field",
                                    label {
                                        class: "form-label",
                                        "Save to file:"
                                    }
                                    input {
                                        class: "form-input",
                                        r#type: "text",
                                        value: "{export_path}",
                                        placeholder: "Enter file path",
                                        oninput: move |evt| export_path.set(evt.value().clone())
                                    }
                                }
                                
                                button {
                                    class: "button-primary",
                                    onclick: save_to_file,
                                    "Save to File"
                                }
                            }
                        } else {
                            div {
                                class: "empty-preview",
                                p { "No preview available. Use the 'Generate Report' button to create a preview." }
                            }
                        }
                    }
                }
            }
        }
    }
}

