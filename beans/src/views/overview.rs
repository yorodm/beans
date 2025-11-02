//! Overview view stub

use dioxus::prelude::*;

#[component]
pub fn OverviewView() -> Element {
    rsx! {
        div { class: "view",
            h1 { "Overview" }
            p { "Implementation in progress" }
        }
    }
}
