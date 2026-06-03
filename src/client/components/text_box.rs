use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum TextBoxType {
    Text,
    Password,
    Email,
    Number { min: i64, max: i64, step: f64 },
}

impl std::fmt::Display for TextBoxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TextBoxType::Text => "text",
            TextBoxType::Password => "password",

            TextBoxType::Number {
                min: _,
                max: _,
                step: _,
            } => "number",
            TextBoxType::Email => "email",
        })
    }
}

#[component]
pub fn TextBox(
    placeholder: String,
    kind: TextBoxType,
    on_input: Option<Callback<Event<FormData>>>,
    disabled_signal: Option<Signal<bool>>,
    required: Option<bool>,
    initial_value: Option<String>,
    id: Option<String>,
    name: Option<String>,
    children: Element,
) -> Element {
    #[css_module("/src/client/assets/css/components/text_box.css")]
    struct Style;

    let disabled = disabled_signal.unwrap_or_else(|| use_signal(|| false));

    let mut number_min: Option<i64> = None;
    let mut number_max: Option<i64> = None;
    let mut number_step: Option<f64> = None;

    if let TextBoxType::Number { min, max, step } = kind {
        number_min = Some(min);
        number_max = Some(max);
        number_step = Some(step);
    }

    rsx! {
        input {
            class: Style::input,
            initial_value,
            required,

            disabled,
            id,
            name,

            placeholder,
            r#type: kind.to_string(),

            min: number_min,
            max: number_max,
            step: number_step,

            oninput: {
                move |e| {
                    if let Some(callback) = on_input {
                        callback.call(e)
                    }
                }
            },

            {children}
        }
    }
}
