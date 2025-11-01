use crate::state::{AppState, AppView};
use freya::prelude::*;

/// Properties for the Ribbon component
#[derive(Props, Clone, PartialEq)]
pub struct RibbonProps {
    /// Application state
    pub state: AppState,
}

/// Ribbon component for navigation
pub fn Ribbon(props: RibbonProps) -> Element {
    let RibbonProps { state } = props;
    let current_view = state.current_view.read();
    let has_ledger = state.ledger_manager.read().is_some();

    rsx! {
        rect {
            width: "100%",
            height: "80px",
            background: "rgb(50, 50, 50)",
            direction: "horizontal",
            padding: "10px",
            shadow: "0 2 5 0 rgba(0, 0, 0, 0.5)",
            
            // Open/Create Ledger button
            rect {
                width: "20%",
                height: "100%",
                padding: "5px",
                main_align: "center",
                cross_align: "center",
                background: if *current_view == AppView::OpenLedger { "rgb(70, 70, 70)" } else { "transparent" },
                border_radius: "5px",
                cursor: "pointer",
                onclick: move |_| state.set_view(AppView::OpenLedger),
                
                label {
                    text_align: "center",
                    color: "white",
                    font_size: "14px",
                    "Open/Create Ledger"
                }
            }
            
            // Overview button (only enabled if ledger is open)
            rect {
                width: "20%",
                height: "100%",
                padding: "5px",
                main_align: "center",
                cross_align: "center",
                background: if *current_view == AppView::Overview { "rgb(70, 70, 70)" } else { "transparent" },
                border_radius: "5px",
                cursor: if has_ledger { "pointer" } else { "not-allowed" },
                opacity: if has_ledger { "1.0" } else { "0.5" },
                onclick: move |_| if has_ledger { state.set_view(AppView::Overview) },
                
                label {
                    text_align: "center",
                    color: "white",
                    font_size: "14px",
                    "Overview"
                }
            }
            
            // Add Entry button (only enabled if ledger is open)
            rect {
                width: "20%",
                height: "100%",
                padding: "5px",
                main_align: "center",
                cross_align: "center",
                background: if *current_view == AppView::AddEntry { "rgb(70, 70, 70)" } else { "transparent" },
                border_radius: "5px",
                cursor: if has_ledger { "pointer" } else { "not-allowed" },
                opacity: if has_ledger { "1.0" } else { "0.5" },
                onclick: move |_| if has_ledger { state.set_view(AppView::AddEntry) },
                
                label {
                    text_align: "center",
                    color: "white",
                    font_size: "14px",
                    "Add Entry"
                }
            }
            
            // Edit Entry button (only enabled if ledger is open)
            rect {
                width: "20%",
                height: "100%",
                padding: "5px",
                main_align: "center",
                cross_align: "center",
                background: if *current_view == AppView::EditEntry { "rgb(70, 70, 70)" } else { "transparent" },
                border_radius: "5px",
                cursor: if has_ledger { "pointer" } else { "not-allowed" },
                opacity: if has_ledger { "1.0" } else { "0.5" },
                onclick: move |_| if has_ledger { state.set_view(AppView::EditEntry) },
                
                label {
                    text_align: "center",
                    color: "white",
                    font_size: "14px",
                    "Edit Entry"
                }
            }
            
            // Export button (only enabled if ledger is open)
            rect {
                width: "20%",
                height: "100%",
                padding: "5px",
                main_align: "center",
                cross_align: "center",
                background: if *current_view == AppView::Export { "rgb(70, 70, 70)" } else { "transparent" },
                border_radius: "5px",
                cursor: if has_ledger { "pointer" } else { "not-allowed" },
                opacity: if has_ledger { "1.0" } else { "0.5" },
                onclick: move |_| if has_ledger { state.set_view(AppView::Export) },
                
                label {
                    text_align: "center",
                    color: "white",
                    font_size: "14px",
                    "Export"
                }
            }
        }
    }
}

