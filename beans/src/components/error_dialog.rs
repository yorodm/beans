use crate::state::AppState;
use freya::prelude::*;

/// Properties for the ErrorDialog component
#[derive(Props, Clone, PartialEq)]
pub struct ErrorDialogProps {
    /// Application state
    pub state: AppState,
}

/// Error dialog component
pub fn ErrorDialog(props: ErrorDialogProps) -> Element {
    let ErrorDialogProps { state } = props;
    let error_message = state.error_message.read();
    
    // Only render if there's an error message
    if error_message.is_none() {
        return rsx! { fragment {} };
    }
    
    rsx! {
        rect {
            position: "absolute",
            top: "50%",
            left: "50%",
            transform: "translate(-50%, -50%)",
            width: "400px",
            padding: "20px",
            background: "rgb(240, 240, 240)",
            border_radius: "5px",
            shadow: "0 4 10 0 rgba(0, 0, 0, 0.3)",
            z_index: "1000",
            
            rect {
                width: "100%",
                direction: "vertical",
                gap: "15px",
                
                label {
                    font_size: "18px",
                    font_weight: "bold",
                    color: "rgb(200, 0, 0)",
                    "Error"
                }
                
                label {
                    font_size: "14px",
                    color: "rgb(50, 50, 50)",
                    "{error_message.as_ref().unwrap()}"
                }
                
                rect {
                    width: "100%",
                    main_align: "end",
                    
                    rect {
                        padding: "8px 15px",
                        background: "rgb(70, 70, 70)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| state.clear_error(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Close"
                        }
                    }
                }
            }
        }
    }
}

