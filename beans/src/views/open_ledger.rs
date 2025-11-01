use crate::state::AppState;
use freya::prelude::*;
use rfd::FileDialog;
use std::path::PathBuf;

/// Properties for the OpenLedger component
#[derive(Props, Clone, PartialEq)]
pub struct OpenLedgerProps {
    /// Application state
    pub state: AppState,
}

/// OpenLedger component for opening or creating a new ledger
pub fn OpenLedger(props: OpenLedgerProps) -> Element {
    let OpenLedgerProps { state } = props;
    let new_ledger_name = use_signal(|| String::new());
    
    // Function to open an existing ledger file
    let open_ledger = move || {
        let file_path = FileDialog::new()
            .add_filter("Bean Ledger", &["bean"])
            .set_directory("/")
            .pick_file();
            
        if let Some(path) = file_path {
            match state.open_ledger(path) {
                Ok(_) => {},
                Err(e) => state.set_error(format!("Failed to open ledger: {}", e)),
            }
        }
    };
    
    // Function to create a new ledger file
    let create_ledger = move || {
        if new_ledger_name.read().is_empty() {
            state.set_error("Please enter a name for the new ledger".to_string());
            return;
        }
        
        let file_path = FileDialog::new()
            .add_filter("Bean Ledger", &["bean"])
            .set_directory("/")
            .set_file_name(&format!("{}.bean", new_ledger_name.read()))
            .save_file();
            
        if let Some(path) = file_path {
            match state.create_ledger(path) {
                Ok(_) => {},
                Err(e) => state.set_error(format!("Failed to create ledger: {}", e)),
            }
        }
    };
    
    rsx! {
        rect {
            width: "100%",
            height: "calc(100% - 80px)",
            padding: "20px",
            direction: "vertical",
            gap: "20px",
            
            // Title
            label {
                font_size: "24px",
                font_weight: "bold",
                color: "rgb(50, 50, 50)",
                "Beans Ledger"
            }
            
            // Open existing ledger section
            rect {
                width: "100%",
                padding: "20px",
                background: "rgb(240, 240, 240)",
                border_radius: "5px",
                direction: "vertical",
                gap: "15px",
                
                label {
                    font_size: "18px",
                    font_weight: "bold",
                    color: "rgb(50, 50, 50)",
                    "Open Existing Ledger"
                }
                
                label {
                    font_size: "14px",
                    color: "rgb(100, 100, 100)",
                    "Select an existing .bean file to open"
                }
                
                rect {
                    width: "100%",
                    main_align: "end",
                    
                    rect {
                        padding: "10px 20px",
                        background: "rgb(70, 130, 180)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| open_ledger(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Browse..."
                        }
                    }
                }
            }
            
            // Create new ledger section
            rect {
                width: "100%",
                padding: "20px",
                background: "rgb(240, 240, 240)",
                border_radius: "5px",
                direction: "vertical",
                gap: "15px",
                
                label {
                    font_size: "18px",
                    font_weight: "bold",
                    color: "rgb(50, 50, 50)",
                    "Create New Ledger"
                }
                
                label {
                    font_size: "14px",
                    color: "rgb(100, 100, 100)",
                    "Enter a name for your new ledger"
                }
                
                rect {
                    width: "100%",
                    height: "40px",
                    padding: "0 10px",
                    background: "white",
                    border: "1px solid rgb(200, 200, 200)",
                    border_radius: "4px",
                    
                    input {
                        width: "100%",
                        height: "100%",
                        font_size: "14px",
                        color: "rgb(50, 50, 50)",
                        placeholder: "Ledger name",
                        value: "{new_ledger_name}",
                        oninput: move |e| new_ledger_name.set(e.value.clone()),
                    }
                }
                
                rect {
                    width: "100%",
                    main_align: "end",
                    
                    rect {
                        padding: "10px 20px",
                        background: "rgb(46, 139, 87)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| create_ledger(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Create"
                        }
                    }
                }
            }
        }
    }
}

