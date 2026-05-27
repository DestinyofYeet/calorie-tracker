use std::rc::Rc;

use dioxus::prelude::*;

use crate::client::components::Button;
use dioxus_free_icons::{
    icons::ld_icons::{LdMenu, LdSquareChevronLeft, LdSquareChevronRight},
    Icon,
};

#[component]
pub fn SideBar(children: Element) -> Element {
    #[css_module("/src/client/assets/css/components/sidebar.css")]
    struct Style;

    let mut is_expanded = use_signal(|| false);

    rsx! {
        div {
            class: Style::toplevel,

            div {
                class: Style::header,

                Button {
                    on_click: move |_| {
                        is_expanded.set(true);
                    },

                    Icon {
                        width: 20,
                        height: 20,
                        fill: "white",
                        icon: LdSquareChevronRight
                    }
                }
            }

            {children}
        }

        div {
            class: Style::sidebarblackout,

            style: if is_expanded() { "display: block;" },
            style: if !is_expanded() { "display: none;" },
        }

        div {
            class: Style::sidebar,

            style: if is_expanded() { "display: block;" },
            style: if !is_expanded() { "display: none;" },

            div {
                class: Style::sidebarheader,

                h3 {
                    "Calorie Tracker"
                }

                Button {
                    on_click: move |_| {
                        is_expanded.set(false);
                    },

                    Icon {
                        width: 20,
                        height: 20,
                        fill: "white",
                        icon: LdSquareChevronLeft,
                    }

                }
            }

            div {
                class: Style::sidebaritems,

                p {
                    "Dashboard"
                }

                p {
                    "Record consumption"
                }

                p {
                    "Manage consumpitions"
                }
            }
        }
    }
}
