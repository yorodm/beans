use crate::state::AppState;
use crate::utils::chart::{BarChart, BarData};
use beans_lib::reporting::TimePeriod;
use chrono::{DateTime, Datelike, Duration, Utc};
use ribir::prelude::*;
use rust_decimal::Decimal;
use std::sync::Arc;

/// Overview view with income vs expenses chart
pub struct OverviewView;

impl OverviewView {
    /// Create a new overview view
    pub fn new(app_state: Arc<AppState>) -> impl WidgetBuilder {
        fn_widget! {
            let ledger = app_state.ledger.clone_reader();
            let report_generator = app_state.report_generator.clone_reader();
            let date_range = app_state.date_range.clone_reader();
            let selected_tags = app_state.selected_tags.clone_reader();
            
            // State for the report data
            let income = Stateful::new(Decimal::ZERO);
            let expenses = Stateful::new(Decimal::ZERO);
            
            // Update the report data when dependencies change
            let _ = pipe!{
                if let (Some(report_gen), Some(_)) = ($report_generator.as_ref(), $ledger.as_ref()) {
                    let (start_date, end_date) = *$date_range;
                    let tags = $selected_tags.clone();
                    
                    // This would be an async call in a real implementation
                    // For now, we'll use placeholder data
                    *$income.write() = Decimal::new(5000, 0);
                    *$expenses.write() = Decimal::new(3500, 0);
                }
            };
            
            @Column {
                h_align: HAlign::Center,
                v_align: VAlign::Center,
                spacing: 20.,
                padding: EdgeInsets::all(20.),
                
                // Date range filter
                @Row {
                    h_align: HAlign::Center,
                    spacing: 20.,
                    margin: EdgeInsets::bottom(20.),
                    
                    // Date range label
                    @Text {
                        text: "Date Range:",
                        font_size: 16.,
                        color: Color::rgb(80, 80, 80),
                    }
                    
                    // Date range buttons
                    @Row {
                        spacing: 10.,
                        
                        // This Month button
                        @FilledButton {
                            on_tap: move |_| {
                                let now = Utc::now();
                                let start = Utc.with_ymd_and_hms(
                                    now.year(),
                                    now.month(),
                                    1,
                                    0,
                                    0,
                                    0,
                                ).unwrap_or(now);
                                
                                app_state.update_date_range(start, now);
                            },
                            @{
                                Label::new("This Month")
                            }
                        }
                        
                        // Last 3 Months button
                        @FilledButton {
                            on_tap: move |_| {
                                let now = Utc::now();
                                let start = now - Duration::days(90);
                                
                                app_state.update_date_range(start, now);
                            },
                            @{
                                Label::new("Last 3 Months")
                            }
                        }
                        
                        // This Year button
                        @FilledButton {
                            on_tap: move |_| {
                                let now = Utc::now();
                                let start = Utc.with_ymd_and_hms(
                                    now.year(),
                                    1,
                                    1,
                                    0,
                                    0,
                                    0,
                                ).unwrap_or(now);
                                
                                app_state.update_date_range(start, now);
                            },
                            @{
                                Label::new("This Year")
                            }
                        }
                    }
                }
                
                // Bar chart
                @{
                    let chart_data = vec![
                        BarData {
                            label: "Income".to_string(),
                            value: *income.read(),
                            color: Color::rgb(100, 200, 100), // Green for income
                        },
                        BarData {
                            label: "Expenses".to_string(),
                            value: *expenses.read(),
                            color: Color::rgb(200, 100, 100), // Red for expenses
                        },
                    ];
                    
                    BarChart::new(chart_data)
                }
                
                // Summary
                @Column {
                    h_align: HAlign::Center,
                    spacing: 10.,
                    margin: EdgeInsets::top(20.),
                    
                    // Net income
                    @Text {
                        text: pipe!{
                            let net = *$income.read() - *$expenses.read();
                            format!("Net: {:.2}", net)
                        },
                        font_size: 18.,
                        font_weight: FontWeight::BOLD,
                        color: pipe!{
                            let net = *$income.read() - *$expenses.read();
                            if net > Decimal::ZERO {
                                Color::rgb(100, 200, 100) // Green for positive
                            } else {
                                Color::rgb(200, 100, 100) // Red for negative
                            }
                        },
                    }
                    
                    // Date range display
                    @Text {
                        text: pipe!{
                            let (start, end) = *$date_range;
                            format!("Period: {} to {}", 
                                start.format("%b %d, %Y"),
                                end.format("%b %d, %Y"))
                        },
                        font_size: 14.,
                        color: Color::rgb(120, 120, 120),
                    }
                }
            }
        }
    }
}
