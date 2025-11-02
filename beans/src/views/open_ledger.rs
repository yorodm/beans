use crate::state::AppState;
use freya::prelude::*;
use rfd::FileDialog;
use std::path::{Path, PathBuf};

/// Properties for the OpenLedger component
#[derive(Props, Clone, PartialEq)]
pub struct OpenLedgerProps {
    /// Application state
    pub state: AppState,
}

/// OpenLedger component for opening or creating a new ledger
pub fn OpenLedger(props: OpenLedgerProps) -> Element {
    let OpenLedgerProps { state } = props;
    
    // Form state for creating a new ledger
    let new_ledger_name = use_signal(|| String::new());
    let new_ledger_path = use_signal(|| String::from("/tmp"));
    
    // Open an existing ledger file
    let open_ledger = move || {
        let file = FileDialog::new()
            .add_filter("Bean Ledger", &["bean"])
            .set_directory("/")
            .pick_file();
            
        if let Some(path) = file {
            match state.open_ledger(path.clone()) {
                Ok(_) => {
                    // Successfully opened the ledger
                }
                Err(e) => {
                    state.set_error(format!("Failed to open ledger: {}", e));
                }
            }
        }
    };
    
    // Browse for a directory to save the new ledger
    let browse_directory = move || {
        let dir = FileDialog::new()
            .set_directory("/")
            .pick_folder();
            
        if let Some(path) = dir {
            new_ledger_path.set(path.to_string_lossy().to_string());
        }
    };
    
    // Create a new ledger
    let create_ledger = move || {
        // Validate form
        if new_ledger_name.read().is_empty() {
            state.set_error("Ledger name is required".to_string());
            return;
        }
        
        // Create the path
        let path_str = format!("{}/{}.bean", new_ledger_path.read(), new_ledger_name.read());
        let path = PathBuf::from(path_str);
        
        // Check if the file already exists
        if path.exists() {
            state.set_error("A ledger with this name already exists".to_string());
            return;
        }
        
        // Create the ledger
        match state.create_ledger(path) {
            Ok(_) => {
                // Successfully created the ledger
            }
            Err(e) => {
                state.set_error(format!("Failed to create ledger: {}", e));
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
                "Open or Create Ledger"
            }
            
            // Open existing ledger
            rect {
                width: "100%",
                max_width: "600px",
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
                    main_align: "center",
                    margin_top: "10px",
                    
                    rect {
                        padding: "10px 20px",
                        background: "rgb(70, 130, 180)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| open_ledger(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Browse for Ledger File"
                        }
                    }
                }
            }
            
            // Create new ledger
            rect {
                width: "100%",
                max_width: "600px",
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
                
                // Ledger name
                rect {
                    width: "100%",
                    direction: "vertical",
                    gap: "5px",
                    
                    label {
                        font_size: "14px",
                        color: "rgb(50, 50, 50)",
                        "Ledger Name *"
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
                            placeholder: "Enter ledger name",
                            value: "{new_ledger_name}",
                            oninput: move |e| new_ledger_name.set(e.value.clone()),
                        }
                    }
                }
                
                // Ledger location
                rect {
                    width: "100%",
                    direction: "vertical",
                    gap: "5px",
                    
                    label {
                        font_size: "14px",
                        color: "rgb(50, 50, 50)",
                        "Save Location"
                    }
                    
                    rect {
                        width: "100%",
                        direction: "horizontal",
                        gap: "10px",
                        
                        rect {
                            width: "calc(100% - 120px)",
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
                                value: "{new_ledger_path}",
                                oninput: move |e| new_ledger_path.set(e.value.clone()),
                            }
                        }
                        
                        rect {
                            width: "110px",
                            height: "40px",
                            background: "rgb(100, 100, 100)",
                            border_radius: "4px",
                            main_align: "center",
                            cross_align: "center",
                            cursor: "pointer",
                            onclick: move |_| browse_directory(),
                            
                            label {
                                color: "white",
                                font_size: "14px",
                                "Browse"
                            }
                        }
                    }
                }
                
                // Create button
                rect {
                    width: "100%",
                    main_align: "end",
                    margin_top: "10px",
                    
                    rect {
                        padding: "10px 20px",
                        background: "rgb(46, 139, 87)",
                        border_radius: "4px",
                        cursor: "pointer",
                        onclick: move |_| create_ledger(),
                        
                        label {
                            color: "white",
                            font_size: "14px",
                            "Create Ledger"
                        }
                    }
                }
            }
        }
    }
}

