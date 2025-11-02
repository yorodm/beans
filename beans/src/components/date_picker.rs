//! Date picker component

use crate::styles;
use freya::prelude::*;

#[component]
pub fn DatePicker(label: String, value: String, on_change: EventHandler<String>) -> Element {
    rsx! {
        rect {
            direction: "vertical",
            spacing: "{styles::spacing::SMALL}",
            width: "100%",

            label {
                font_size: "{styles::fonts::NORMAL}",
                color: "{styles::colors::TEXT_PRIMARY}",
                "{label}"
            }

            Input {
                value: value.clone(),
                placeholder: "YYYY-MM-DD",
                onchange: move |new_value| {
                    on_change.call(new_value);
                }
            }
        }
    }
}

