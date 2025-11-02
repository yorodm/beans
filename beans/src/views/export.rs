//! Export view for exporting ledger data to different formats

use crate::components::filter_panel::FilterPanel;
use crate::state::{AppState, View};
use beans_lib::prelude::*;
use beans_lib::reporting::{ExportFormat, ReportGenerator};
use dioxus::prelude::*;
use std::path::PathBuf;
use chrono::Utc;

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
        // Clone all signals for use in the async block
        let app_state = app_state.clone();
        let format = format.clone();
        let preview_content = preview_content.clone();
        let has_preview = has_preview.clone();
        
        // Spawn the async operation
        spawn(async move {
            // Extract all data we need while holding the read lock
            let (entries, start_date, end_date, tags, export_format) = {
                let state = app_state.read();
                
                if state.ledger_manager.is_none() {
                    drop(state);
                    app_state.write().set_error("No ledger is open".to_string());
                    return;
                }
                
                let manager = state.ledger_manager.as_ref().unwrap();
                
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
                
                // Get the start and end dates from the filter
                let start_date = filter.start_date.unwrap_or_else(|| {
                    // Default to 30 days ago if no start date
                    Utc::now() - chrono::Duration::days(30)
                });
                
                let end_date = filter.end_date.unwrap_or_else(Utc::now);
                
                // Get the tags from the filter
                let tags = if filter.tags.is_empty() {
                    None
                } else {
                    Some(filter.tags.clone())
                };
                
                // Convert format string to ExportFormat
                let export_format = match format().as_str() {
                    "json" => ExportFormat::Json,
                    "csv" => ExportFormat::Csv,
                    _ => {
                        drop(state);
                        app_state.write().set_error("Unsupported export format".to_string());
                        return;
                    }
                };
                
                // Get entries (we need to do this while we have the manager reference)
                let entries = match manager.list_entries(&filter) {
                    Ok(e) => e,
                    Err(err) => {
                        drop(state);
                        app_state.write().set_error(format!("Failed to list entries: {}", err));
                        return;
                    }
                };
                
                (entries, start_date, end_date, tags, export_format)
            };
            
            // Now we can work with the extracted data without holding the read lock
            
            // Create a temporary LedgerManager for the ReportGenerator
            let temp_manager = match LedgerManager::new_in_memory() {
                Ok(mut m) => {
                    // Add the cloned entries to the in-memory manager
                    for entry in &entries {
                        if let Err(e) = m.add_entry(entry.clone()) {
                            app_state.write().set_error(format!("Failed to prepare report data: {}", e));
                            return;
                        }
                    }
                    m
                },
                Err(e) => {
                    app_state.write().set_error(format!("Failed to create temporary ledger: {}", e));
                    return;
                }
            };
            
            // Create a ReportGenerator from the temporary LedgerManager
            let report_generator = ReportGenerator::new(&temp_manager);
            
            // Generate the report
            let report = match report_generator.tagged_report(start_date, end_date, tags).await {
                Ok(r) => r,
                Err(e) => {
                    app_state.write().set_error(format!("Failed to generate report: {}", e));
                    return;
                }
            };
            
            // Export the report
            match report_generator.export_tagged_report(&report, export_format) {
                Ok(content) => {
                    preview_content.set(content);
                    has_preview.set(true);
                    app_state.write().set_success("Report generated successfully".to_string());
                }
                Err(e) => {
                    app_state.write().set_error(format!("Failed to export report: {}", e));
                    has_preview.set(false);
                }
            }
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

