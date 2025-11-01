use crate::state::{AppState, View};
use ribir::prelude::*;
use std::sync::Arc;

/// Ribbon button style
fn ribbon_button_style() -> Style {
    Style::new()
        .padding(EdgeInsets::all(8.))
        .margin(EdgeInsets::horizontal(4.))
        .border_radius(4.)
        .background(Color::rgb(240, 240, 240))
        .hover(|s| s.background(Color::rgb(220, 220, 220)))
        .active(|s| s.background(Color::rgb(200, 200, 200)))
}

/// Active ribbon button style
fn active_ribbon_button_style() -> Style {
    ribbon_button_style()
        .background(Color::rgb(200, 220, 250))
        .hover(|s| s.background(Color::rgb(180, 200, 240)))
        .active(|s| s.background(Color::rgb(160, 180, 230)))
}

/// Ribbon toolbar component
pub struct Ribbon;

impl Ribbon {
    /// Create a new ribbon toolbar
    pub fn new(app_state: Arc<AppState>) -> impl WidgetBuilder {
        fn_widget! {
            let current_view = app_state.current_view.clone_reader();
            
            @Row {
                h_align: HAlign::Center,
                padding: EdgeInsets::all(8.),
                background: Color::rgb(250, 250, 250),
                border_width: EdgeInsets::bottom(1.),
                border_color: Color::rgb(220, 220, 220),
                
                // Open/Create Ledger button
                @FilledButton {
                    style: pipe!{
                        if *$current_view == View::Welcome {
                            active_ribbon_button_style()
                        } else {
                            ribbon_button_style()
                        }
                    },
                    on_tap: move |_| {
                        app_state.navigate_to(View::Welcome);
                    },
                    @{
                        Label::new("Open/Create")
                    }
                }
                
                // Overview button
                @FilledButton {
                    style: pipe!{
                        if *$current_view == View::Overview {
                            active_ribbon_button_style()
                        } else {
                            ribbon_button_style()
                        }
                    },
                    on_tap: move |_| {
                        app_state.navigate_to(View::Overview);
                    },
                    @{
                        Label::new("Overview")
                    }
                }
                
                // Add Entry button
                @FilledButton {
                    style: pipe!{
                        if *$current_view == View::AddEntry {
                            active_ribbon_button_style()
                        } else {
                            ribbon_button_style()
                        }
                    },
                    on_tap: move |_| {
                        app_state.navigate_to(View::AddEntry);
                    },
                    @{
                        Label::new("Add Entry")
                    }
                }
                
                // Edit Entry button
                @FilledButton {
                    style: pipe!{
                        if *$current_view == View::EditEntry {
                            active_ribbon_button_style()
                        } else {
                            ribbon_button_style()
                        }
                    },
                    on_tap: move |_| {
                        app_state.navigate_to(View::EditEntry);
                    },
                    @{
                        Label::new("Edit Entry")
                    }
                }
                
                // Export button
                @FilledButton {
                    style: pipe!{
                        if *$current_view == View::Export {
                            active_ribbon_button_style()
                        } else {
                            ribbon_button_style()
                        }
                    },
                    on_tap: move |_| {
                        app_state.navigate_to(View::Export);
                    },
                    @{
                        Label::new("Export")
                    }
                }
            }
        }
    }
}
