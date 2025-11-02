//! Export view for exporting ledger data to different formats

use crate::components::filter_panel::FilterPanel;
use crate::state::{AppState, View};
use beans_lib::prelude::*;
use dioxus::prelude::*;
use std::path::PathBuf;

/// Export View
/// 
/// This view provides functionality to export ledger data with:
/// - Filter configuration (reuse FilterPanel)
/// - Format selection (JSON/CSV radio buttons)
/// - Preview and save area
#[component]
pub fn ExportView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    
    // Local state
    let mut format = use_signal(|| "json".to_string());
    let mut export_path = use_signal(|| {
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
    let mut preview_content = use_signal(String::new);
    let mut has_preview = use_signal(|| false);
    
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
        let state = app_state.read();
        
        if let Some(manager) = &state.ledger_manager {
            // Create filter from current state
            let mut filter = EntryFilter::new();
            
            if let Some(start) = state.filter.date_range.start {
                filter.start_date = Some(start);
            }
            
            if let Some(end) = state.filter.date_range.end {
                filter.end_date = Some(end);
            }
            
            for tag in &state.filter.tags {
                filter.tags.push(tag.clone());
            }
            
            // Generate report
            let report_result = match format().as_str() {
                "json" => manager.export_json(&filter),
                "csv" => manager.export_csv(&filter),
                _ => Err(BeansError::validation("Unsupported export format")),
            };
            
            match report_result {
                Ok(content) => {
                    preview_content.set(content);
                    has_preview.set(true);
                    drop(state);
                    app_state.write().set_success("Report generated successfully".to_string());
                }
                Err(e) => {
                    drop(state);
                    app_state.write().set_error(format!("Failed to generate report: {}", e));
                    has_preview.set(false);
                }
            }
        } else {
            drop(state);
            app_state.write().set_error("No ledger is open".to_string());
        }
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
        match std::fs::write(&path, preview_content()) {
            Ok(_) => {
                app_state.write().set_success(format!("Report saved to {}", path.display()));
            }
            Err(e) => {
                app_state.write().set_error(format!("Failed to save report: {}", e));
            }
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
