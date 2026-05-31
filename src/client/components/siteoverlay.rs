use dioxus::prelude::*;

use crate::client::{
    components::{Blackout, Button},
    Routes,
};
use dioxus_free_icons::{
    icons::ld_icons::{LdSquareChevronLeft, LdSquareChevronRight},
    Icon,
};

#[component]
pub fn SiteOverlay(children: Element) -> Element {
    #[css_module("/src/client/assets/css/components/siteoverlay.css")]
    struct Style;

    let mut is_expanded = use_signal(|| false);

    let title = "Calorie Tracker";

    rsx! {
        div {
            div { class: Style::header,

                Button {
                    on_click: move |_| {
                        is_expanded.set(true);
                    },

                    Icon {
                        width: 20,
                        height: 20,
                        fill: "white",
                        icon: LdSquareChevronRight,
                    }
                }

                Link { to: Routes::Landing {}, {title} }
            }

            {children}
        }

        Blackout {
            enabled: is_expanded,
        }

        div {
            class: Style::sidebar,

            style: if is_expanded() { "display: block;" },
            style: if !is_expanded() { "display: none;" },

            div { class: Style::sidebarheader,

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

                h3 {
                    Link { to: Routes::ApplicationStart {}, {title} }
                }
            }

            div { class: Style::sidebaritems,

                p { "Dashboard" }

                Link {

                    to: Routes::ConsumptionAdd {},
                    onclick: move |_| {
                        is_expanded.set(false);
                    },

                    "[DEV] Record consumption"
                }

                Link {

                    to: Routes::ConsumablesManage {},
                    onclick: move |_| {
                        is_expanded.set(false);
                    },

                    "Manage consumables"
                }
            }
        }
    }
}
