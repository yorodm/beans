//! Ledger selection view - entry point to the application

use crate::state::AppState;
use crate::styles;
use beans_lib::BeansError;
use freya::prelude::*;
use std::path::PathBuf;

#[component]
pub fn LedgerSelectionView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    // Local state for input fields
    let mut open_path = use_signal(String::new);
    let mut create_path = use_signal(String::new);

    // Handler for opening an existing ledger
    let handle_open = move |_| {
        app_state.write().clear_messages();
        let path_str = open_path().clone();
        
        if path_str.trim().is_empty() {
            app_state
                .write()
                .set_error("Please enter a file path".to_string());
            return;
        }

        // Auto-append .bean extension if not present
        let final_path = if path_str.ends_with(".bean") {
            path_str.to_string()
        } else {
            format!("{}.bean", path_str)
        };

        let path = PathBuf::from(&final_path);

        // Check if file exists
        if !path.exists() {
            app_state
                .write()
                .set_error(format!("File does not exist: {}", final_path));
            return;
        }

        // Attempt to open the ledger
        let open_ledger: Result<String, BeansError> = match app_state.write().open_ledger(path) {
            Ok(_) => Ok(format!("Successfully opened ledger: {}", final_path)),
            Err(e) => Err(e),
        };
        match open_ledger {
            Ok(m) => app_state.write().set_success(m),
            Err(e) => app_state.write().set_error(e.to_string()),
        }
    };

    // Handler for creating a new ledger
    let handle_create = move |_| {
        app_state.write().clear_messages();
        let path_str = create_path().clone();
        
        if path_str.trim().is_empty() {
            app_state
                .write()
                .set_error("Please enter a file path".to_string());
            return;
        }

        // Auto-append .bean extension if not present
        let final_path = if path_str.ends_with(".bean") {
            path_str.to_string()
        } else {
            format!("{}.bean", path_str)
        };

        let path = PathBuf::from(&final_path);

        // Check if file already exists
        if path.exists() {
            app_state.write().set_error(format!(
                "File already exists: {}. Use 'Open Ledger' instead.",
                final_path
            ));
            return;
        }

        // Attempt to create the ledger
        let create_ledger = match app_state.write().create_ledger(path) {
            Ok(_) => Ok(format!("Successfully created ledger: {}", final_path)),
            Err(e) => Err(e),
        };
        match create_ledger {
            Ok(m) => app_state.write().set_success(m),
            Err(e) => app_state.write().set_error(e.to_string()),
        }
    };

    rsx! {
        ScrollView {
            width: "100%",
            height: "fill",
            show_scrollbar: true,

            rect {
                width: "100%",
                padding: "{styles::spacing::XLARGE}",
                direction: "vertical",
                spacing: "{styles::spacing::LARGE}",

                // Title
                label {
                    font_size: "{styles::fonts::TITLE}",
                    font_weight: "bold",
                    color: "{styles::colors::TEXT_PRIMARY}",
                    "Welcome to Beans 🫘"
                }

                label {
                    font_size: "{styles::fonts::MEDIUM}",
                    color: "{styles::colors::TEXT_SECONDARY}",
                    "Manage your personal finances with ease. Open an existing ledger or create a new one to get started."
                }

                // Display error or success messages
                if let Some(error) = &app_state.read().error_message {
                    rect {
                        width: "100%",
                        padding: "{styles::spacing::MEDIUM}",
                        background: "{styles::colors::ERROR}",
                        corner_radius: "{styles::radius::SMALL}",

                        label {
                            color: "white",
                            font_size: "{styles::fonts::NORMAL}",
                            "{error}"
                        }
                    }
                }
                
                if let Some(success) = &app_state.read().success_message {
                    rect {
                        width: "100%",
                        padding: "{styles::spacing::MEDIUM}",
                        background: "{styles::colors::SUCCESS}",
                        corner_radius: "{styles::radius::SMALL}",

                        label {
                            color: "white",
                            font_size: "{styles::fonts::NORMAL}",
                            "{success}"
                        }
                    }
                }

                // Two-card layout for open vs create
                rect {
                    width: "100%",
                    direction: "horizontal",
                    spacing: "{styles::spacing::LARGE}",

                    // Card for opening existing ledger
                    rect {
                        width: "50%",
                        padding: "{styles::spacing::LARGE}",
                        background: "white",
                        corner_radius: "{styles::radius::MEDIUM}",
                        shadow: "0 2 4 0 {styles::colors::SHADOW}",
                        direction: "vertical",
                        spacing: "{styles::spacing::MEDIUM}",

                        label {
                            font_size: "{styles::fonts::LARGE}",
                            font_weight: "bold",
                            color: "{styles::colors::TEXT_PRIMARY}",
                            "📂 Open Existing Ledger"
                        }

                        label {
                            font_size: "{styles::fonts::NORMAL}",
                            color: "{styles::colors::TEXT_SECONDARY}",
                            "Open a ledger file that you've previously created."
                        }

                        label {
                            font_size: "{styles::fonts::NORMAL}",
                            color: "{styles::colors::TEXT_PRIMARY}",
                            margin: "{styles::spacing::MEDIUM} 0 0 0",
                            "File Path:"
                        }

                        Input {
                            value: open_path().clone(),
                            placeholder: "e.g., my_ledger or /path/to/ledger.bean",
                            onchange: move |e| open_path.set(e),
                        }

                        label {
                            font_size: "{styles::fonts::SMALL}",
                            color: "{styles::colors::TEXT_SECONDARY}",
                            "Tip: .bean extension will be added automatically if missing"
                        }

                        Button {
                            onclick: handle_open,
                            label { "Open Ledger" }
                        }
                    }

                    // Card for creating new ledger
                    rect {
                        width: "50%",
                        padding: "{styles::spacing::LARGE}",
                        background: "white",
                        corner_radius: "{styles::radius::MEDIUM}",
                        shadow: "0 2 4 0 {styles::colors::SHADOW}",
                        direction: "vertical",
                        spacing: "{styles::spacing::MEDIUM}",

                        label {
                            font_size: "{styles::fonts::LARGE}",
                            font_weight: "bold",
                            color: "{styles::colors::TEXT_PRIMARY}",
                            "✨ Create New Ledger"
                        }

                        label {
                            font_size: "{styles::fonts::NORMAL}",
                            color: "{styles::colors::TEXT_SECONDARY}",
                            "Start fresh with a new ledger file."
                        }

                        label {
                            font_size: "{styles::fonts::NORMAL}",
                            color: "{styles::colors::TEXT_PRIMARY}",
                            margin: "{styles::spacing::MEDIUM} 0 0 0",
                            "File Path:"
                        }

                        Input {
                            value: create_path().clone(),
                            placeholder: "e.g., my_ledger or /path/to/ledger.bean",
                            onchange: move |e| create_path.set(e),
                        }

                        label {
                            font_size: "{styles::fonts::SMALL}",
                            color: "{styles::colors::TEXT_SECONDARY}",
                            "Tip: .bean extension will be added automatically if missing"
                        }

                        Button {
                            onclick: handle_create,
                            label { "Create Ledger" }
                        }
                    }
                }

                // Help text at the bottom
                rect {
                    width: "100%",
                    padding: "{styles::spacing::LARGE}",
                    background: "#f9f9f9",
                    corner_radius: "{styles::radius::MEDIUM}",
                    direction: "vertical",
                    spacing: "{styles::spacing::MEDIUM}",

                    label {
                        font_size: "{styles::fonts::MEDIUM}",
                        font_weight: "bold",
                        color: "{styles::colors::TEXT_PRIMARY}",
                        "💡 Getting Started"
                    }

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_SECONDARY}",
                        "• Ledger files use the .bean extension"
                    }

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_SECONDARY}",
                        "• You can specify a relative path (e.g., my_ledger) or absolute path"
                    }

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_SECONDARY}",
                        "• Parent directories will be created automatically if needed"
                    }

                    label {
                        font_size: "{styles::fonts::NORMAL}",
                        color: "{styles::colors::TEXT_SECONDARY}",
                        "• Once opened, you can add entries, view reports, and export data"
                    }
                }
            }
        }
    }
}

