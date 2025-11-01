use crate::state::{AppState, EntryFilter};
use beans_lib::{
    models::EntryType,
    reporting::{IncomeExpenseReport, TimePeriod},
};
use chrono::{Local, NaiveDate};
use freya::prelude::*;
use rust_decimal::Decimal;

/// Properties for the Overview component
#[derive(Props, Clone, PartialEq)]
pub struct OverviewProps {
    /// Application state
    pub state: AppState,
}

/// Overview component for displaying income vs expenses
pub fn Overview(props: OverviewProps) -> Element {
    let OverviewProps { state } = props;
    let filter = state.filter.read().clone();
    
    // Date range filter
    let start_date = use_signal(|| filter.start_date);
    let end_date = use_signal(|| filter.end_date);
    
    // Tag filter
    let tag_filter = use_signal(|| filter.tags.join(", "));
    
    // Get the report data
    let report_data = use_memo(move || {
        let report_gen = state.report_generator.read();
        if let Some(report_gen) = report_gen.as_ref() {
            // Create a filter for the report
            let mut db_filter = beans_lib::database::EntryFilter::default();
            
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
            
            // Generate the report
            match report_gen.generate_income_expense_report(TimePeriod::All, db_filter) {
                Ok(report) => Some(report),
                Err(e) => {
                    state.set_error(format!("Failed to generate report: {}", e));
                    None
                }
            }
        } else {
            None
        }
    });
    
    // Apply the filter
    let apply_filter = move || {
        let new_filter = EntryFilter {
            start_date: *start_date.read(),
            end_date: *end_date.read(),
            tags: tag_filter
                .read()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            currency: None,
            entry_type: None,
        };
        
        state.apply_filter(new_filter);
    };
    
    // Format a date for display
    let format_date = |date: Option<NaiveDate>| {
        date.map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "".to_string())
    };
    
    // Get the current date
    let current_date = Local::now().date_naive().format("%Y-%m-%d").to_string();
    
    // Calculate the maximum value for the graph
    let (income, expenses, max_value) = if let Some(report) = report_data.read().as_ref() {
        let income = report.total_income;
        let expenses = report.total_expenses;
        let max = income.max(expenses);
        (income, expenses, max)
    } else {
        (Decimal::ZERO, Decimal::ZERO, Decimal::ONE)
    };
    
    // Calculate the bar heights (as percentages)
    let income_height = if max_value > Decimal::ZERO {
        (income * Decimal::from(80) / max_value).to_string()
    } else {
        "0".to_string()
    };
    
    let expenses_height = if max_value > Decimal::ZERO {
        (expenses * Decimal::from(80) / max_value).to_string()
    } else {
        "0".to_string()
    };
    
    rsx! {
        rect {
            width: "100%",
            height: "calc(100% - 80px)",
            padding: "20px",
            direction: "vertical",
            gap: "20px",
            
            // Title and current date
            rect {
                width: "100%",
                direction: "horizontal",
                main_align: "space-between",
                
                label {
                    font_size: "24px",
                    font_weight: "bold",
                    color: "rgb(50, 50, 50)",
                    "Overview"
                }
                
                label {
                    font_size: "16px",
                    color: "rgb(100, 100, 100)",
                    "Current Date: {current_date}"
                }
            }
            
            // Filter controls
            rect {
                width: "100%",
                padding: "15px",
                background: "rgb(240, 240, 240)",
                border_radius: "5px",
                direction: "vertical",
                gap: "15px",
                
                label {
                    font_size: "16px",
                    font_weight: "bold",
                    color: "rgb(50, 50, 50)",
                    "Filter Options"
                }
                
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
                
                // Apply button
                rect {
                    width: "100%",
                    main_align: "end",
                    
                    rect {
                        padding: "8px 15px",
                        background: "rgb(70, 130, 180)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| apply_filter(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Apply Filter"
                        }
                    }
                }
            }
            
            // Graph
            rect {
                width: "100%",
                height: "calc(100% - 200px)",
                min_height: "300px",
                padding: "20px",
                background: "rgb(240, 240, 240)",
                border_radius: "5px",
                direction: "vertical",
                gap: "15px",
                
                label {
                    font_size: "16px",
                    font_weight: "bold",
                    color: "rgb(50, 50, 50)",
                    "Income vs Expenses"
                }
                
                // Bar graph
                rect {
                    width: "100%",
                    height: "calc(100% - 30px)",
                    direction: "horizontal",
                    main_align: "center",
                    gap: "50px",
                    
                    // Income bar
                    rect {
                        width: "100px",
                        height: "100%",
                        direction: "vertical",
                        main_align: "end",
                        cross_align: "center",
                        gap: "10px",
                        
                        // Bar
                        rect {
                            width: "100%",
                            height: "{income_height}%",
                            min_height: "1px",
                            background: "rgb(46, 139, 87)",
                            border_radius: "5px 5px 0 0",
                        }
                        
                        // Label
                        label {
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            "Income"
                        }
                        
                        // Amount
                        label {
                            font_size: "14px",
                            font_weight: "bold",
                            color: "rgb(46, 139, 87)",
                            "{income}"
                        }
                    }
                    
                    // Expenses bar
                    rect {
                        width: "100px",
                        height: "100%",
                        direction: "vertical",
                        main_align: "end",
                        cross_align: "center",
                        gap: "10px",
                        
                        // Bar
                        rect {
                            width: "100%",
                            height: "{expenses_height}%",
                            min_height: "1px",
                            background: "rgb(178, 34, 34)",
                            border_radius: "5px 5px 0 0",
                        }
                        
                        // Label
                        label {
                            font_size: "14px",
                            color: "rgb(50, 50, 50)",
                            "Expenses"
                        }
                        
                        // Amount
                        label {
                            font_size: "14px",
                            font_weight: "bold",
                            color: "rgb(178, 34, 34)",
                            "{expenses}"
                        }
                    }
                }
            }
        }
    }
}

