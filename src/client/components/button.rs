use dioxus::prelude::*;

#[component]
pub fn Button(
    on_click: Option<Callback<MouseEvent>>,
    disabled: Option<Signal<bool>>,
    children: Element,
) -> Element {
    #[css_module("/src/client/assets/css/components/button.css")]
    struct Style;

    let disabled = disabled.unwrap_or_else(|| use_signal(|| false));

    rsx! {
        button {
            class: Style::button,
            onclick: move |evt| {
                if let Some(callback) = on_click {
                    callback.call(evt)
                }
            },

            disabled,

            {children}
        }
    }
}
