use dioxus::prelude::*;

#[component]
pub fn LabeledTextInput(
    mut value: Signal<String>,
    label: String,
    mut error: Signal<Option<String>>,
    required: bool,
    input_type: Option<String>,
) -> Element {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    rsx! {
        div { class: "flex-1 grid gap-2 items-center",
            label { class: "font-bold",
                span { class: "flex items-center",
                    "{label}"
                    if let Some(msg) = error() {
                        span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                    }
                }
            }
            input {
                class: "border border-gray-400 rounded py-1 px-2 flex-1",
                r#type: "{input_type}",
                value: "{value}",
                required: "{required}",
                oninput: move |e| {
                    // update the passed-in signal
                    value.set(e.value());
                }
            }
        }
    }
}
