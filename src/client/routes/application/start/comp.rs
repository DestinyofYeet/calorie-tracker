use dioxus::prelude::*;

use crate::client::components::SideBar;

#[component]
pub fn ApplicationStart() -> Element {
    #[css_module("/src/client/assets/css/application/start.css")]
    struct Style;

    rsx! {
        SideBar {

            p {
                "Main content"
            }

            p {
                "This is some very big horizontal text, lorem ipsum blub blub blub blub "
            }
        }
    }
}
