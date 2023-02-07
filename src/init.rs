use wasm_bindgen::JsValue;

pub fn init() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    Ok(())
}
