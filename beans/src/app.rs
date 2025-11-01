use crate::state::{AppState, View};
use crate::ui::{
    EntryFormView, EntryListView, ExportView, OverviewView, Ribbon, WelcomeView,
};
use ribir::prelude::*;
use std::sync::Arc;

/// Main application component
pub struct App;

impl App {
    /// Run the application
    pub fn run() {
        // Initialize the application state
        let app_state = Arc::new(AppState::new());
        
        // Create the main application widget
        let app_widget = fn_widget! {
            let current_view = app_state.current_view.clone_reader();
            
            @Column {
                // Ribbon toolbar
                @{
                    Ribbon::new(app_state.clone())
                }
                
                // Main content area
                @Container {
                    flex: 1.,
                    
                    // Switch between views based on current_view
                    @pipe!{
                        match *$current_view {
                            View::Welcome => WelcomeView::new(app_state.clone()).into_widget(),
                            View::Overview => OverviewView::new(app_state.clone()).into_widget(),
                            View::AddEntry => EntryFormView::new(app_state.clone()).into_widget(),
                            View::EditEntry => EntryListView::new(app_state.clone()).into_widget(),
                            View::Export => ExportView::new(app_state.clone()).into_widget(),
                        }
                    }
                }
            }
        };
        
        // Run the application
        ribir::app::run(app_widget);
    }
}
