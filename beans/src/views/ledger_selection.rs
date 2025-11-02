//! Ledger selection view - entry point to the application

use crate::state::AppState;
use beans_lib::BeansError;
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn LedgerSelectionView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    // Local state for input fields
    let mut open_path = use_signal(|| None::<String>);
    let mut create_path = use_signal(|| None::<String>);

    // Handler for opening an existing ledger
    let handle_open = move |_| {
        app_state.write().clear_messages();
        if let Some(path_str) = open_path.read().clone() {
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
            let open_ledger: Result<String, BeansError> = match app_state.write().open_ledger(path)
            {
                Ok(_) => Ok(format!("Successfully opened ledger: {}", final_path)),
                Err(e) => Err(e),
            };
            match open_ledger {
                Ok(m) => app_state.write().set_success(m),
                Err(e) => app_state.write().set_error(e.to_string()),
            }
        }
    };

    // Handler for creating a new ledger
    let handle_create = move |_| {
        app_state.write().clear_messages();
        if let Some(path_str) = create_path.read().clone() {
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
        }
    };

    rsx! {
        div { class: "view",
            h1 { "Welcome to Beans 🫘" }
            p { class: "subtitle",
                "Manage your personal finances with ease. Open an existing ledger or create a new one to get started."
            }

            // Display error or success messages
            if let Some(error) = &app_state.read().error_message {
                div { class: "message error-message", "{error}" }
            }
            if let Some(success) = &app_state.read().success_message {
                div { class: "message success-message", "{success}" }
            }

            // Two-card layout for open vs create
            div { class: "ledger-actions",

                // Card for opening existing ledger
                div { class: "action-card",
                    h2 { "📂 Open Existing Ledger" }
                    p { "Open a ledger file that you've previously created." }

                    div { class: "form-group",
                        label { r#for: "open-path", "File Path" }
                        input {
                            id: "open-path",
                            r#type: "file",
                            placeholder: "e.g., my_ledger or /path/to/ledger.bean",
                            onchange: move |evt| {
                                if let Some(file) = evt.files().get(0) {
                                    open_path.set(Some(file.name().to_string()));
                                }
                            },
                        }
                        small { style: "color: #666; font-size: 12px;",
                            "Tip: .bean extension will be added automatically if missing"
                        }
                    }

                    button {
                        class: "button-primary",
                        onclick: handle_open,
                        "Open Ledger"
                    }
                }

                // Card for creating new ledger
                div { class: "action-card",
                    h2 { "✨ Create New Ledger" }
                    p { "Start fresh with a new ledger file." }

                    div { class: "form-group",
                        label { r#for: "create-path", "File Path" }
                        input {
                            id: "create-path",
                            r#type: "file",
                            placeholder: "e.g., my_ledger or /path/to/ledger.bean",
                            onchange: move |evt| {
                                if let Some(file) = evt.files().get(0) {
                                    create_path.set(Some(file.name().to_string()));
                                }
                            }
                        }
                        small { style: "color: #666; font-size: 12px;",
                            "Tip: .bean extension will be added automatically if missing"
                        }
                    }

                    button {
                        class: "button-primary",
                        onclick: handle_create,
                        "Create Ledger"
                    }
                }
            }

            // Help text at the bottom
            div { style: "margin-top: 32px; padding: 16px; background: #f9f9f9; border-radius: 6px;",
                h3 { "💡 Getting Started" }
                ul { style: "margin-left: 20px; margin-top: 8px;",
                    li { style: "margin-bottom: 6px;",
                        "Ledger files use the ", code { ".bean" }, " extension"
                    }
                    li { style: "margin-bottom: 6px;",
                        "You can specify a relative path (e.g., ", code { "my_ledger" }, ") or absolute path"
                    }
                    li { style: "margin-bottom: 6px;",
                        "Parent directories will be created automatically if needed"
                    }
                    li { style: "margin-bottom: 6px;",
                        "Once opened, you can add entries, view reports, and export data"
                    }
                }
            }
        }
    }
}
