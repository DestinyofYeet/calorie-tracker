use dioxus::prelude::*;

#[derive(Eq, PartialEq, Clone)]
pub struct SelectValue {
    key: String,
    value: String,
}

impl From<(&str, &str)> for SelectValue {
    fn from(value: (&str, &str)) -> Self {
        Self {
            key: value.0.to_string(),
            value: value.1.to_string(),
        }
    }
}

impl From<(String, String)> for SelectValue {
    fn from(value: (String, String)) -> Self {
        Self {
            key: value.0,
            value: value.1,
        }
    }
}

#[component]
pub fn Select<T>(
    id: Option<String>,
    name: Option<String>,
    required: bool,
    options: Vec<T>,
) -> Element
where
    T: Into<SelectValue> + Clone + 'static + PartialEq,
{
    #[css_module("/src/client/assets/css/components/select.css")]
    struct Style;

    let values: Vec<SelectValue> = options.into_iter().map(Into::into).collect();

    rsx! {
        select { class: Style::select, id, required, name,

            for value in values {
                option { value: value.value, {value.key} }
            }
        }
    }
}
