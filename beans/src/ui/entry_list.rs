use crate::state::AppState;
use ribir::prelude::*;
use std::sync::Arc;

/// Entry list view for editing entries
pub struct EntryListView;

impl EntryListView {
    /// Create a new entry list view
    pub fn new(app_state: Arc<AppState>) -> impl WidgetBuilder {
        fn_widget! {
            @Column {
                h_align: HAlign::Center,
                v_align: VAlign::Center,
                spacing: 20.,
                padding: EdgeInsets::all(20.),
                
                // Title
                @Text {
                    text: "Edit Entries",
                    font_size: 24.,
                    font_weight: FontWeight::BOLD,
                    color: Color::rgb(60, 60, 60),
                    margin: EdgeInsets::bottom(20.),
                }
                
                // Placeholder for the entry list
                @Text {
                    text: "Entry list will be implemented in the next phase",
                    font_size: 16.,
                    color: Color::rgb(100, 100, 100),
                }
            }
        }
    }
}
