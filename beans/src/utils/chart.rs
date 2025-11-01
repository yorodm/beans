use ribir::prelude::*;
use rust_decimal::Decimal;

/// Bar chart data
#[derive(Debug, Clone)]
pub struct BarData {
    /// Label for the bar
    pub label: String,
    /// Value for the bar
    pub value: Decimal,
    /// Color for the bar
    pub color: Color,
}

/// Bar chart component
pub struct BarChart;

impl BarChart {
    /// Create a new bar chart
    pub fn new(data: Vec<BarData>) -> impl WidgetBuilder {
        fn_widget! {
            // Find the maximum value for scaling
            let max_value = data.iter()
                .map(|d| d.value)
                .max()
                .unwrap_or(Decimal::ONE);
            
            // Convert to f32 for rendering
            let max_value_f32 = max_value.to_f32().unwrap_or(1.0);
            
            @Column {
                width: 400.,
                height: 300.,
                padding: EdgeInsets::all(20.),
                background: Color::rgb(250, 250, 250),
                border_radius: 8.,
                border_width: EdgeInsets::all(1.),
                border_color: Color::rgb(220, 220, 220),
                
                // Chart title
                @Text {
                    text: "Income vs Expenses",
                    font_size: 18.,
                    font_weight: FontWeight::BOLD,
                    color: Color::rgb(60, 60, 60),
                    margin: EdgeInsets::bottom(20.),
                    h_align: HAlign::Center,
                }
                
                // Bar container
                @Row {
                    height: 200.,
                    spacing: 40.,
                    h_align: HAlign::Center,
                    v_align: VAlign::End,
                    
                    // Generate bars
                    @for bar in data {
                        @Column {
                            width: 80.,
                            spacing: 8.,
                            
                            // Bar
                            @Container {
                                height: pipe!{
                                    let value_f32 = bar.value.to_f32().unwrap_or(0.0);
                                    let height = (value_f32 / max_value_f32) * 180.0;
                                    height.max(1.0) // Ensure at least 1px height
                                },
                                width: 60.,
                                background: bar.color,
                                border_radius: EdgeInsets::top(4.),
                            }
                            
                            // Bar label
                            @Text {
                                text: bar.label.clone(),
                                font_size: 14.,
                                color: Color::rgb(80, 80, 80),
                                h_align: HAlign::Center,
                            }
                            
                            // Bar value
                            @Text {
                                text: format!("{:.2}", bar.value),
                                font_size: 12.,
                                color: Color::rgb(120, 120, 120),
                                h_align: HAlign::Center,
                            }
                        }
                    }
                }
            }
        }
    }
}
