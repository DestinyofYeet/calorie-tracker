use dioxus::prelude::*;

#[component]
pub fn TextBox(
    placeholder: String,
    kind: Option<String>,
    on_input: Option<Callback<Event<FormData>>>,
    disabled_signal: Option<Signal<bool>>,
    id: Option<String>,
) -> Element {
    #[css_module("/src/client/assets/css/components/text_box.css")]
    struct Style;

    let disabled = disabled_signal.unwrap_or_else(|| use_signal(|| false));

    rsx! {
        input {
            class: Style::input,
            initial_value: "",

            disabled,
            id,

            placeholder,
            r#type: if kind.is_some() { kind.unwrap() },

            oninput: {
                move |e| {
                    if let Some(callback) = on_input {
                        callback.call(e)
                    }
                }
            },
        }
    }
}
