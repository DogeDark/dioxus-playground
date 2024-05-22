use crate::components::{Header, RightPane, Tab};
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use model::*;

mod components;
mod ws;

#[cfg(not(debug_assertions))]
const SOCKET_URI: &str = "ws://localhost:3000/ws";

#[cfg(debug_assertions)]
const SOCKET_URI: &str = "ws://localhost:3000/ws";

#[cfg(not(debug_assertions))]
const BUILT_URI: &str = "https://play.dioxuslabs.com/built/";

#[cfg(debug_assertions)]
const BUILT_URI: &str = "http://localhost:3000/built/";

const SNIPPET_WELCOME: &str = include_str!("snippets/welcome.rs");

fn main() {
    dioxus_logger::init(Level::WARN).expect("failed to start logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut is_compiling = use_signal(|| false);
    let mut built_page_uri = use_signal(|| None);
    let mut compiler_messages = use_signal(Vec::<String>::new);
    let mut current_tab = use_signal(|| Tab::Page);

    // Change tab automatically
    use_memo(move || {
        if built_page_uri().is_none() {
            current_tab.set(Tab::Logs);
        } else {
            current_tab.set(Tab::Page);
        }
    });

    let socket_tx = ws::start_socket(is_compiling, built_page_uri, compiler_messages);

    // Once the element has mounted, startup `ace` editor.
    let on_editor_mount = move |_| async move {
        let code = format!(
            r#"
            let editor = ace.edit("editor");
            editor.setTheme("ace/theme/github");

            let RustMode = ace.require("ace/mode/rust").Mode;
            editor.session.setMode(new RustMode());

            editor.setValue(`{SNIPPET_WELCOME}`, -1);

            // Set a global so other evals can acces it.
            window.editorGlobal = editor;
            "#
        );
        eval(&code);
    };

    // Send a request to compile code.
    let on_run = move |_| {
        spawn(async move {
            if is_compiling() {
                return;
            }
            is_compiling.set(true);
            built_page_uri.set(None);
            compiler_messages.clear();
            compiler_messages.push("Starting build...".to_string());

            let mut eval = eval(
                r#"
                let text = window.editorGlobal.getValue();
                dioxus.send(text);
                "#,
            );

            // TODO: Error Handling
            let val = eval.recv().await.unwrap().as_str().unwrap().to_string();
            socket_tx.send(SocketMessage::CompileRequest(val));
        });
    };

    rsx! {
        div {
            id: "pane-container",
            div {
                id: "left-pane",
                Header {
                    is_compiling: is_compiling(),
                    on_run,
                }
                div {
                    id: "editor",
                    onmounted: on_editor_mount,
                }
            }

            RightPane {
                current_tab: current_tab(),
                built_page_uri: built_page_uri(),
                compiler_messages: compiler_messages(),
            }
        }
    }
}
