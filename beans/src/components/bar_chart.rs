//! Simple bar chart component for income vs expenses

use crate::styles;
use freya::prelude::*;
use rust_decimal::Decimal;

#[component]
pub fn BarChart(income: Decimal, expenses: Decimal, currency: String) -> Element {
    // Calculate heights for visual representation (max 200px)
    let max_value = income.max(expenses);
    
    let income_height = if max_value > Decimal::ZERO {
        let ratio = income / max_value;
        (ratio * Decimal::from(200)).to_string()
    } else {
        "0".to_string()
    };

    let expenses_height = if max_value > Decimal::ZERO {
        let ratio = expenses / max_value;
        (ratio * Decimal::from(200)).to_string()
    } else {
        "0".to_string()
    };

    let balance = income - expenses;
    let balance_color = if balance >= Decimal::ZERO {
        styles::colors::SUCCESS
    } else {
        styles::colors::ERROR
    };

    rsx! {
        rect {
            width: "100%",
            padding: "{styles::spacing::LARGE}",
            background: "white",
            corner_radius: "{styles::radius::MEDIUM}",
            shadow: "0 2 8 0 {styles::colors::SHADOW}",

            // Title
            label {
                font_size: "{styles::fonts::LARGE}",
                font_weight: "bold",
                color: "{styles::colors::TEXT_PRIMARY}",
                margin: "0 0 {styles::spacing::LARGE} 0",
                "Income vs Expenses ({currency})"
            }

            // Chart bars
            rect {
                width: "100%",
                height: "250",
                direction: "horizontal",
                main_align: "center",
                cross_align: "end",
                spacing: "{styles::spacing::XLARGE}",
                padding: "{styles::spacing::MEDIUM}",

                // Income bar wrapper
                rect {
                    width: "120",
                    direction: "vertical",
                    cross_align: "center",
                    spacing: "{styles::spacing::SMALL}",

                    // Bar
                    rect {
                        width: "80",
                        height: "{income_height}",
                        background: "{styles::colors::INCOME}",
                        corner_radius: "{styles::radius::SMALL}",
                        main_align: "center",
                        cross_align: "center",

                        label {
                            color: "white",
                            font_size: "{styles::fonts::SMALL}",
                            font_weight: "bold",
                            "{income}"
                        }
                    }

                    // Label
                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        margin: "{styles::spacing::SMALL} 0 0 0",
                        "Income"
                    }
                }

                // Expenses bar wrapper
                rect {
                    width: "120",
                    direction: "vertical",
                    cross_align: "center",
                    spacing: "{styles::spacing::SMALL}",

                    // Bar
                    rect {
                        width: "80",
                        height: "{expenses_height}",
                        background: "{styles::colors::EXPENSE}",
                        corner_radius: "{styles::radius::SMALL}",
                        main_align: "center",
                        cross_align: "center",

                        label {
                            color: "white",
                            font_size: "{styles::fonts::SMALL}",
                            font_weight: "bold",
                            "{expenses}"
                        }
                    }

                    // Label
                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        margin: "{styles::spacing::SMALL} 0 0 0",
                        "Expenses"
                    }
                }
            }

            // Summary
            rect {
                width: "100%",
                direction: "vertical",
                spacing: "{styles::spacing::SMALL}",
                padding: "{styles::spacing::MEDIUM} 0 0 0",

                label {
                    font_size: "{styles::fonts::NORMAL}",
                    color: "{styles::colors::TEXT_PRIMARY}",
                    "Total Income: {currency} {income}"
                }
                label {
                    font_size: "{styles::fonts::NORMAL}",
                    color: "{styles::colors::TEXT_PRIMARY}",
                    "Total Expenses: {currency} {expenses}"
                }
                label {
                    font_size: "{styles::fonts::NORMAL}",
                    font_weight: "bold",
                    color: "{balance_color}",
                    "Balance: {currency} {balance}"
                }
            }
        }
    }
}
