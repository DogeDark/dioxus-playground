use dioxus::prelude::*;
use crate::{bindings::monaco, examples};

const ARROW_DOWN: &str = asset!("/public/arrow-down.svg");

#[component]
pub fn Header(
    pane_left_width: Signal<Option<i32>>,
    pane_right_width: Signal<Option<i32>>,
    on_run: EventHandler,
) -> Element {
    let mut examples_open = use_signal(|| false);

    rsx! {
        div {
            id: "dxp-header",
            // Left pane header
            div {
                id: "dxp-header-left",
                style: if let Some(val) = pane_left_width() { "width:{val}px;" },

                button {
                    id: "dxp-run-btn",
                    class: "dxp-ctrl-btn",
                    onclick: move |_| on_run.call(()),
                    "Run"
                }
                div {
                    id: "dxp-examples-btn-container",
                    button {
                        id: "dxp-examples-btn",
                        class: "dxp-ctrl-btn",
                        class: if examples_open() { "dxp-open" },
                        onclick: move |_| examples_open.set(!examples_open()),
                        "Examples"
                        img { src: ARROW_DOWN, height: "16px", width: "16px" }
                    }

                    if examples_open() {
                        div {
                            id: "dxp-examples-dropdown",

                            for snippet in examples::SNIPPETS {
                                button {
                                    onclick: move |_| {
                                        examples_open.set(false);
                                        monaco::set_current_model_value(snippet.1);
                                    },
                                    "{snippet.0}"
                                }
                            }
                        }
                    }
                }
                div {
                    id: "dxp-header-left-divider",
                }
                button {
                    class: "dxp-ctrl-btn dxp-file-btn dxp-selected-file",
                    "main.rs"
                }
                button {
                    class: "dxp-ctrl-btn dxp-file-btn",
                    "Cargo.toml"
                }
            }

            // Right pane header
            div {
                id: "dxp-header-right",
                style: if let Some(val) = pane_right_width() { "width:{val}px;" } else { "".to_string() },
                button {
                    id: "dxp-share-btn",
                    class: "dxp-ctrl-btn",
                    "Share"
                }
            }
        }
    }
}

// const SPINNER: &str = asset!("/public/spinner.svg");

// #[component]
// pub fn Header(is_compiling: bool, queue_position: Option<u32>, on_run: EventHandler) -> Element {
//     let on_clear = move |_| {
//         bindings::monaco::clear_current_model_value();
//     };

//     rsx! {
//         div { id: "dxp-header",

//             button {
//                 id: "dxp-run-button",
//                 class: if is_compiling { "disabled" },

//                 onclick: move |_| on_run.call(()),
//                 if is_compiling {
//                     img { class: "dxp-spinner", src: "{SPINNER}" }

//                     if let Some(pos) = queue_position {
//                         "#{pos}"
//                     }
//                 } else {
//                     "Run"
//                 }
//             }

//             h1 { id: "dxp-title", "Dioxus Playground" }

//             button { id: "dxp-clear-button", onclick: on_clear, "Clear" }
//         }
//     }
// }
