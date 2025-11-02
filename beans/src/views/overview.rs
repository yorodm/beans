//! Overview view with dashboard and entry list

use crate::components::bar_chart::BarChart;
use crate::components::filter_panel::FilterPanel;
use crate::state::{AppState, View};
use crate::styles;
use beans_lib::prelude::*;
use chrono::Utc;
use freya::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Overview View - Dashboard with income/expense chart and entry list
#[component]
pub fn OverviewView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    // Get current date
    let current_date = Utc::now().format("%Y-%m-%d").to_string();

    // Calculate income and expenses from entries
    let (income, expenses, currency_totals) = {
        let state = app_state.read();
        let entries = &state.entries;

        // Track totals by currency
        let mut currency_map: HashMap<String, (Decimal, Decimal)> = HashMap::new();

        // Process all entries
        for entry in entries {
            let currency = entry.currency_code();
            let amount = entry.amount();

            let totals = currency_map.entry(currency).or_insert((Decimal::ZERO, Decimal::ZERO));

            match entry.entry_type() {
                EntryType::Income => totals.0 += amount,
                EntryType::Expense => totals.1 += amount,
            }
        }

        // Get the first currency's totals or default to zero
        let default_currency = state.entries.first().map(|e| e.currency_code()).unwrap_or_else(|| "USD".to_string());
        let (income, expenses) = currency_map.get(&default_currency).cloned().unwrap_or((Decimal::ZERO, Decimal::ZERO));

        (income, expenses, currency_map)
    };

    // Handle filter apply
    let on_filter_apply = move |_| {};

    // Handle entry selection for editing
    let mut select_entry = move |id: Uuid| {
        let mut state = app_state.write();
        state.selected_entry = Some(id);
        state.set_view(View::EditEntry);
    };

    // Get default currency
    let default_currency = app_state.read().entries.first()
        .map(|e| e.currency_code())
        .unwrap_or_else(|| "USD".to_string());

    let entries = app_state.read().entries.clone();

    rsx! {
        rect {
            width: "100%",
            height: "fill",
            padding: "{styles::spacing::LARGE}",
            direction: "vertical",
            spacing: "{styles::spacing::LARGE}",

            // Header with date
            rect {
                direction: "horizontal",
                main_align: "spaceBetween",
                cross_align: "center",

                label {
                    font_size: "{styles::fonts::TITLE}",
                    font_weight: "bold",
                    color: "{styles::colors::TEXT_PRIMARY}",
                    "Overview"
                }

                label {
                    font_size: "{styles::fonts::NORMAL}",
                    color: "{styles::colors::TEXT_SECONDARY}",
                    "Today: {current_date}"
                }
            }

            // Success/error messages
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
                height: "fill",
                direction: "horizontal",
                spacing: "{styles::spacing::LARGE}",

                // Left sidebar with filters
                FilterPanel {
                    on_apply: on_filter_apply
                }

                // Main area with chart and entries
                ScrollView {
                    width: "fill",
                    height: "fill",
                    show_scrollbar: true,

                    rect {
                        direction: "vertical",
                        spacing: "{styles::spacing::LARGE}",
                        width: "100%",

                        // Chart section
                        if app_state.read().entries.is_empty() {
                            rect {
                                width: "100%",
                                padding: "{styles::spacing::XLARGE}",
                                background: "white",
                                corner_radius: "{styles::radius::MEDIUM}",
                                shadow: "0 2 4 0 {styles::colors::SHADOW}",
                                direction: "vertical",
                                main_align: "center",
                                cross_align: "center",
                                spacing: "{styles::spacing::MEDIUM}",

                                label {
                                    font_size: "{styles::fonts::LARGE}",
                                    color: "{styles::colors::TEXT_SECONDARY}",
                                    "No entries found"
                                }

                                label {
                                    font_size: "{styles::fonts::NORMAL}",
                                    color: "{styles::colors::TEXT_SECONDARY}",
                                    "Add your first entry to see the chart"
                                }

                                Button {
                                    onclick: move |_| app_state.write().set_view(View::AddEntry),
                                    label { "Add Entry" }
                                }
                            }
                        } else {
                            BarChart {
                                income: income,
                                expenses: expenses,
                                currency: default_currency.clone()
                            }

                            // Show other currencies if present
                            if currency_totals.len() > 1 {
                                rect {
                                    width: "100%",
                                    padding: "{styles::spacing::LARGE}",
                                    background: "white",
                                    corner_radius: "{styles::radius::MEDIUM}",
                                    shadow: "0 2 4 0 {styles::colors::SHADOW}",
                                    direction: "vertical",
                                    spacing: "{styles::spacing::MEDIUM}",

                                    label {
                                        font_size: "{styles::fonts::LARGE}",
                                        font_weight: "bold",
                                        color: "{styles::colors::TEXT_PRIMARY}",
                                        "Other Currencies"
                                    }

                                    for (currency, (curr_income, curr_expenses)) in currency_totals.iter() {
                                        if currency != &default_currency {
                                            label {
                                                font_size: "{styles::fonts::NORMAL}",
                                                color: "{styles::colors::TEXT_PRIMARY}",
                                                "{currency}: Income {curr_income}, Expenses {curr_expenses}, Balance {curr_income - curr_expenses}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Recent entries section
                        rect {
                            width: "100%",
                            padding: "{styles::spacing::LARGE}",
                            background: "white",
                            corner_radius: "{styles::radius::MEDIUM}",
                            shadow: "0 2 4 0 {styles::colors::SHADOW}",
                            direction: "vertical",
                            spacing: "{styles::spacing::MEDIUM}",

                            label {
                                font_size: "{styles::fonts::LARGE}",
                                font_weight: "bold",
                                color: "{styles::colors::TEXT_PRIMARY}",
                                "Recent Entries"
                            }

                            if app_state.read().entries.is_empty() {
                                label {
                                    font_size: "{styles::fonts::NORMAL}",
                                    color: "{styles::colors::TEXT_SECONDARY}",
                                    "No entries found."
                                }
                            } else {
                                // Show up to 10 most recent entries
                                for entry in entries.iter().take(10) {
                                    rect {
                                        width: "100%",
                                        padding: "{styles::spacing::MEDIUM}",
                                        background: match entry.entry_type() {
                                            EntryType::Income => "#e8f5e9",
                                            EntryType::Expense => "#ffebee",
                                        },
                                        corner_radius: "{styles::radius::SMALL}",
                                        direction: "horizontal",
                                        main_align: "spaceBetween",
                                        cross_align: "center",

                                        rect {
                                            direction: "vertical",
                                            spacing: "{styles::spacing::TINY}",

                                            label {
                                                font_size: "{styles::fonts::NORMAL}",
                                                font_weight: "bold",
                                                color: "{styles::colors::TEXT_PRIMARY}",
                                                "{entry.date().format(\"%Y-%m-%d\")} • {entry.name()}"
                                            }

                                            label {
                                                font_size: "{styles::fonts::SMALL}",
                                                color: "{styles::colors::TEXT_SECONDARY}",
                                                "{entry.entry_type()} • {entry.amount()} {entry.currency_code()}"
                                            }

                                            if !entry.tags().is_empty() {
                                                rect {
                                                    direction: "horizontal",
                                                    spacing: "{styles::spacing::TINY}",

                                                    for tag in entry.tags() {
                                                        rect {
                                                            padding: "2 {styles::spacing::SMALL}",
                                                            background: "{styles::colors::INFO}",
                                                            corner_radius: "{styles::radius::SMALL}",

                                                            label {
                                                                font_size: "{styles::fonts::SMALL}",
                                                                color: "white",
                                                                "{tag.name()}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        Button {
                                            onclick: move |_| select_entry(entry.id()),
                                            label { "Edit" }
                                        }
                                    }
                                }

                                if app_state.read().entries.len() > 10 {
                                    label {
                                        font_size: "{styles::fonts::SMALL}",
                                        color: "{styles::colors::TEXT_SECONDARY}",
                                        margin: "{styles::spacing::MEDIUM} 0 0 0",
                                        "Showing 10 of {app_state.read().entries.len()} entries. Use the Edit Entry view to see more."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

