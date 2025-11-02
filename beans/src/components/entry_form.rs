//! Entry form component stub - to be fully implemented

use dioxus::prelude::*;
use beans_lib::prelude::*;

#[component]
pub fn EntryForm(
    entry: Option<LedgerEntry>,
    on_save: EventHandler<LedgerEntry>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "entry-form",
            p { "Entry form component - implementation in progress" }
        }
    }
}
