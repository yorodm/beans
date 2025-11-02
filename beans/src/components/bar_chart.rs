//! Simple bar chart component for income vs expenses

use dioxus::prelude::*;
use rust_decimal::Decimal;

#[component]
pub fn BarChart(income: Decimal, expenses: Decimal, currency: String) -> Element {
    // Calculate percentages for visual representation
    let total = income + expenses;
    let max_value = income.max(expenses);

    let income_height = if max_value > Decimal::ZERO {
        ((income / max_value) * Decimal::from(100)).to_string()
    } else {
        "0".to_string()
    };

    let expenses_height = if max_value > Decimal::ZERO {
        ((expenses / max_value) * Decimal::from(100)).to_string()
    } else {
        "0".to_string()
    };

    rsx! {
        div {
            class: "bar-chart",

            div {
                class: "chart-title",
                "Income vs Expenses ({currency})"
            }

            div {
                class: "chart-container",

                // Income bar
                div {
                    class: "bar-wrapper",

                    div {
                        class: "bar-label",
                        "Income"
                    }

                    div {
                        class: "bar income-bar",
                        style: "height: {income_height}%",

                        div {
                            class: "bar-value",
                            "{income}"
                        }
                    }
                }

                // Expenses bar
                div {
                    class: "bar-wrapper",

                    div {
                        class: "bar-label",
                        "Expenses"
                    }

                    div {
                        class: "bar expenses-bar",
                        style: "height: {expenses_height}%",

                        div {
                            class: "bar-value",
                            "{expenses}"
                        }
                    }
                }
            }

            // Summary
            div {
                class: "chart-summary",
                div { "Total Income: {currency} {income}" }
                div { "Total Expenses: {currency} {expenses}" }
                div {
                    class: if income > expenses { "positive" } else { "negative" },
                    "Balance: {currency} {income - expenses}"
                }
            }
        }
    }
}
