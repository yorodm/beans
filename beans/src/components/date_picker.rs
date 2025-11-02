//! Date picker component

use dioxus::prelude::*;

#[component]
pub fn DatePicker(
    label: String,
    value: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "date-picker",
            
            label {
                class: "date-picker-label",
                "{label}"
            }
            
            input {
                r#type: "date",
                class: "date-picker-input",
                value: "{value}",
                oninput: move |evt| {
                    on_change.call(evt.value().clone());
                }
            }
        }
    }
}
