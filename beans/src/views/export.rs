//! Export view for exporting ledger data to different formats

use crate::components::filter_panel::FilterPanel;
use crate::state::AppState;
use crate::styles;
use beans_lib::prelude::*;
use beans_lib::reporting::{ExportFormat, ReportGenerator};
use chrono::Utc;
use freya::prelude::*;
use std::path::PathBuf;

/// Export View - Export ledger data in various formats
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

    // Handle filter apply
    let on_filter_apply = move |_| {};

    // Handle format change
    let mut on_format_change = move |new_format: String| {
        format.set(new_format.clone());

        // Update export path extension
        if let Some(path) = &app_state.read().ledger_path {
            let mut new_path = path.clone();
            new_path.set_extension(new_format);
            export_path.set(new_path.to_string_lossy().to_string());
        }
    };

    // Generate and save report
    let generate_report = move |_| {
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

        // Set default dates if not specified
        let start_date = filter.start_date.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30));
        let end_date = filter.end_date.unwrap_or_else(Utc::now);
        let tags = filter.tags.clone();

        // Query entries
        let entries = match manager.query_entries(filter) {
            Ok(entries) => entries,
            Err(e) => {
                drop(state);
                app_state.write().set_error(format!("Failed to query entries: {}", e));
                return;
            }
        };

        // Create report generator
        let generator = match ReportGenerator::new(manager.ledger()) {
            Ok(g) => g,
            Err(e) => {
                drop(state);
                app_state.write().set_error(format!("Failed to create report generator: {}", e));
                return;
            }
        };

        // Generate report
        let export_format = match format().as_str() {
            "json" => ExportFormat::Json,
            "csv" => ExportFormat::Csv,
            _ => ExportFormat::Json,
        };

        let report = match generator.generate_report(start_date, end_date, &tags, export_format) {
            Ok(r) => r,
            Err(e) => {
                drop(state);
                app_state.write().set_error(format!("Failed to generate report: {}", e));
                return;
            }
        };

        // Save report
        let path_str = export_path();
        if path_str.trim().is_empty() {
            drop(state);
            app_state.write().set_error("Please enter an export path".to_string());
            return;
        }

        let path = PathBuf::from(&path_str);
        match std::fs::write(&path, report) {
            Ok(_) => {
                drop(state);
                app_state.write().set_success(format!("Report exported successfully to: {}", path_str));
            }
            Err(e) => {
                drop(state);
                app_state.write().set_error(format!("Failed to write report: {}", e));
            }
        }
    };

    rsx! {
        rect {
            width: "100%",
            height: "fill",
            padding: "{styles::spacing::LARGE}",
            direction: "vertical",
            spacing: "{styles::spacing::LARGE}",

            label {
                font_size: "{styles::fonts::TITLE}",
                font_weight: "bold",
                color: "{styles::colors::TEXT_PRIMARY}",
                "Export Ledger"
            }

            // Messages
            if let Some(success) = &app_state.read().success_message {
                rect {
                    width: "100%",
                    padding: "{styles::spacing::MEDIUM}",
                    background: "{styles::colors::SUCCESS}",
                    corner_radius: "{styles::radius::SMALL}",
                    label {
                        color: "white",
                        font_size: "{styles::fonts::NORMAL}",
                        "{success}"
                    }
                }
            }

            if let Some(error) = &app_state.read().error_message {
                rect {
                    width: "100%",
                    padding: "{styles::spacing::MEDIUM}",
                    background: "{styles::colors::ERROR}",
                    corner_radius: "{styles::radius::SMALL}",
                    label {
                        color: "white",
                        font_size: "{styles::fonts::NORMAL}",
                        "{error}"
                    }
                }
            }

            // Main content
            rect {
                width: "100%",
                direction: "horizontal",
                spacing: "{styles::spacing::LARGE}",

                FilterPanel {
                    on_apply: on_filter_apply
                }

                // Export configuration
                rect {
                    width: "fill",
                    direction: "vertical",
                    spacing: "{styles::spacing::LARGE}",

                    // Format selection
                    rect {
                        direction: "vertical",
                        spacing: "{styles::spacing::SMALL}",
                        width: "100%",

                        label {
                            font_size: "{styles::fonts::MEDIUM}",
                            font_weight: "bold",
                            color: "{styles::colors::TEXT_PRIMARY}",
                            "Export Format:"
                        }

                        rect {
                            direction: "horizontal",
                            spacing: "{styles::spacing::MEDIUM}",

                            Button {
                                onclick: move |_| on_format_change("json".to_string()),
                                label {
                                    color: if format() == "json" { "white" } else { "{styles::colors::TEXT_PRIMARY}" },
                                    "JSON"
                                }
                            }

                            Button {
                                onclick: move |_| on_format_change("csv".to_string()),
                                label {
                                    color: if format() == "csv" { "white" } else { "{styles::colors::TEXT_PRIMARY}" },
                                    "CSV"
                                }
                            }
                        }
                    }

                    // Export path
                    rect {
                        direction: "vertical",
                        spacing: "{styles::spacing::SMALL}",
                        width: "100%",

                        label {
                            font_size: "{styles::fonts::NORMAL}",
                            color: "{styles::colors::TEXT_PRIMARY}",
                            "Export Path:"
                        }

                        Input {
                            value: export_path().clone(),
                            placeholder: "Enter file path",
                            onchange: move |e| export_path.set(e),
                        }
                    }

                    // Export button
                    Button {
                        onclick: generate_report,
                        label { "Export Report" }
                    }

                    // Instructions
                    rect {
                        padding: "{styles::spacing::MEDIUM}",
                        background: "#f9f9f9",
                        corner_radius: "{styles::radius::SMALL}",
                        direction: "vertical",
                        spacing: "{styles::spacing::SMALL}",

                        label {
                            font_size: "{styles::fonts::SMALL}",
                            color: "{styles::colors::TEXT_SECONDARY}",
                            "• Use the filter panel to narrow down entries"
                        }

                        label {
                            font_size: "{styles::fonts::SMALL}",
                            color: "{styles::colors::TEXT_SECONDARY}",
                            "• Select your preferred export format (JSON or CSV)"
                        }

                        label {
                            font_size: "{styles::fonts::SMALL}",
                            color: "{styles::colors::TEXT_SECONDARY}",
                            "• Click 'Export Report' to save the file"
                        }
                    }
                }
            }
        }
    }
}

