use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bindings/monaco.js")]
extern "C" {
    #[wasm_bindgen(js_name = initMonaco)]
    fn init_monaco(vs_path_prefix: &str, element_id: &str, initial_snippet: &str);

    #[wasm_bindgen(js_name = getCurrentModelValue)]
    pub fn get_current_model_value() -> String;

    #[wasm_bindgen(js_name = setCurrentModelvalue)]
    pub fn set_current_model_value(value: &str);

    #[wasm_bindgen(js_name = isReady)]
    pub fn is_ready() -> bool;

    #[wasm_bindgen(js_name = registerPasteAsRSX)]
    fn register_paste_as_rsx(convertHtmlToRSX: &Closure<dyn Fn(String) -> Option<String>>);
}

pub fn init(vs_path_prefix: &str, element_id: &str, initial_snippet: &str) {
    init_monaco(vs_path_prefix, element_id, initial_snippet);
    register_paste_as_rsx_action();
}

fn register_paste_as_rsx_action() {
    let callback = Closure::new(|html: String| {
        let dom = rsx_rosetta::Dom::parse(&html).ok()?;
        let rsx_callbody = rsx_rosetta::rsx_from_html(&dom);
        dioxus_autofmt::write_block_out(&rsx_callbody)
    });

    register_paste_as_rsx(&callback);
    callback.forget();
}
