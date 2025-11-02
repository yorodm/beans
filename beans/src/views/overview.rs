//! Overview view with dashboard and entry list

use crate::components::bar_chart::BarChart;
use crate::components::filter_panel::FilterPanel;
use crate::state::{AppState, View};
use beans_lib::prelude::*;
use chrono::Utc;
use dioxus::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Overview View
///
/// This view provides a dashboard with:
/// - Current date display
/// - Bar chart showing income vs expenses
/// - Filter panel for date and tag filtering
/// - List of recent entries
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
    let on_filter_apply = move |_| {
        // Filtering is handled by the FilterPanel component
        // We just need to refresh the view
    };

    // Handle entry selection for editing
    let mut select_entry = move |id: Uuid| {
        let mut state = app_state.write();
        state.selected_entry = Some(id);
        state.set_view(View::EditEntry);
    };

    // Get default currency outside the rsx! macro
    let default_currency = app_state.read().entries.first()
        .map(|e| e.currency_code())
        .unwrap_or_else(|| "USD".to_string());

    let entries = app_state.read().entries.clone().into_iter();

    rsx! {
        div {
            class: "view overview-view",

            // Header with date
            div {
                class: "overview-header",
                h1 { "Overview" }
                div { class: "current-date", "Today: {current_date}" }
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
                class: "overview-content",

                // Left sidebar with filters
                div {
                    class: "overview-sidebar",
                    FilterPanel {
                        on_apply: on_filter_apply
                    }
                }

                // Main area with chart and entries
                div {
                    class: "overview-main",

                    // Chart section
                    div {
                        class: "chart-section",

                        if app_state.read().entries.is_empty() {
                            div {
                                class: "empty-state",
                                p { "No entries found. Add your first entry to see the chart." }
                                button {
                                    class: "button-primary",
                                    onclick: move |_| app_state.write().set_view(View::AddEntry),
                                    "Add Entry"
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
                                div {
                                    class: "other-currencies",
                                    h3 { "Other Currencies" }

                                    for (currency, (curr_income, curr_expenses)) in currency_totals.iter() {
                                        if currency != &default_currency {
                                            div {
                                                class: "currency-summary",
                                                p { "{currency}: Income {curr_income}, Expenses {curr_expenses}, Balance {curr_income - curr_expenses}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Recent entries section
                    div {
                        class: "entries-section",
                        h2 { "Recent Entries" }

                        if app_state.read().entries.is_empty() {
                            p { class: "empty-message", "No entries found." }
                        } else {
                            table {
                                class: "entries-table",

                                thead {
                                    tr {
                                        th { "Date" }
                                        th { "Name" }
                                        th { "Type" }
                                        th { "Amount" }
                                        th { "Currency" }
                                        th { "Tags" }
                                        th { "Actions" }
                                    }
                                }

                                tbody {
                                    // Show up to 10 most recent entries
                                    for entry in entries.take(10) {
                                        tr {
                                            class: match entry.entry_type() {
                                                EntryType::Income => "income-row",
                                                EntryType::Expense => "expense-row",
                                            },

                                            td { "{entry.date().format(\"%Y-%m-%d\")}" }
                                            td { "{entry.name()}" }
                                            td { "{entry.entry_type()}" }
                                            td { "{entry.amount()}" }
                                            td { "{entry.currency_code()}" }
                                            td {
                                                class: "tag-cell",
                                                for tag in entry.tags() {
                                                    span { class: "tag-pill", "{tag.name()}" }
                                                }
                                            }
                                            td {
                                                button {
                                                    class: "button-small",
                                                    onclick: move |_| select_entry(entry.id()),
                                                    "Edit"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if app_state.read().entries.len() > 10 {
                                p { class: "more-entries", "Showing 10 of {app_state.read().entries.len()} entries. Use the Edit Entry view to see more." }
                            }
                        }
                    }
                }
            }
        }
    }
}
