//! Entry form component stub - to be fully implemented

use beans_lib::prelude::*;
use dioxus::prelude::*;

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
