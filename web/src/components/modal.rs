use dioxus::prelude::*;

#[component]
pub fn Modal(
    icon: Element,
    title: String,
    text: String,
    on_ok: EventHandler,
) -> Element {
    rsx! {
        // Background
        div {
            id: "dxp-modal-bg",

            div {
                id: "dxp-modal",

                // Modal header with optional icon
                div {
                    id: "dxp-modal-header",
                    {icon}
                    h4 {
                        id: "dxp-modal-title",
                        "{title}"
                    }
                }

                // Modal description text
                p {
                    id: "dxp-modal-text",
                    "{text}"
                }

                // ok button
                button {
                    id: "dxp-modal-ok-btn",
                    onclick: move |_| on_ok.call(()),
                    "I understand"
                }
            }
        }
    }
}
