//! Export view stub

use dioxus::prelude::*;

#[component]
pub fn ExportView() -> Element {
    rsx! {
        div { class: "view",
            h1 { "Export" }
            p { "Implementation in progress" }
        }
    }
}
