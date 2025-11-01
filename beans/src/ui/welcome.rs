use crate::state::AppState;
use ribir::prelude::*;
use rfd::FileDialog;
use std::path::PathBuf;
use std::sync::Arc;

/// Welcome view for opening or creating a ledger
pub struct WelcomeView;

impl WelcomeView {
    /// Create a new welcome view
    pub fn new(app_state: Arc<AppState>) -> impl WidgetBuilder {
        fn_widget! {
            @Column {
                h_align: HAlign::Center,
                v_align: VAlign::Center,
                spacing: 20.,
                
                // Title
                @Text {
                    text: "Welcome to Beans",
                    font_size: 24.,
                    font_weight: FontWeight::BOLD,
                    color: Color::rgb(60, 60, 60),
                }
                
                // Subtitle
                @Text {
                    text: "A multi-platform ledger application",
                    font_size: 16.,
                    color: Color::rgb(100, 100, 100),
                    margin: EdgeInsets::bottom(20.),
                }
                
                // Open ledger button
                @FilledButton {
                    padding: EdgeInsets::all(12.),
                    margin: EdgeInsets::all(8.),
                    on_tap: move |e| {
                        let app_state_clone = app_state.clone();
                        let window = e.window();
                        
                        spawn_local(async move {
                            if let Some(path) = open_file_dialog() {
                                if let Err(err) = app_state_clone.open_ledger(path).await {
                                    show_error_dialog(window, &format!("Error opening ledger: {}", err));
                                }
                            }
                        });
                    },
                    @{
                        Label::new("Open Existing Ledger")
                    }
                }
                
                // Create new ledger button
                @FilledButton {
                    padding: EdgeInsets::all(12.),
                    margin: EdgeInsets::all(8.),
                    on_tap: move |e| {
                        let app_state_clone = app_state.clone();
                        let window = e.window();
                        
                        spawn_local(async move {
                            if let Some(path) = save_file_dialog() {
                                if let Err(err) = app_state_clone.create_ledger(path).await {
                                    show_error_dialog(window, &format!("Error creating ledger: {}", err));
                                }
                            }
                        });
                    },
                    @{
                        Label::new("Create New Ledger")
                    }
                }
            }
        }
    }
}

/// Open a file dialog for selecting a ledger file
fn open_file_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Beans Ledger", &["bean"])
        .set_title("Open Ledger File")
        .pick_file()
}

/// Open a file dialog for saving a new ledger file
fn save_file_dialog() -> Option<PathBuf> {
    let mut path = FileDialog::new()
        .add_filter("Beans Ledger", &["bean"])
        .set_title("Create New Ledger File")
        .save_file()?;
    
    // Ensure the file has the .bean extension
    if let Some(ext) = path.extension() {
        if ext != "bean" {
            path.set_extension("bean");
        }
    } else {
        path.set_extension("bean");
    }
    
    Some(path)
}

/// Show an error dialog
fn show_error_dialog(window: WindowId, message: &str) {
    // In a real implementation, this would show a proper error dialog
    // For now, we'll just log the error
    log::error!("Error: {}", message);
}
