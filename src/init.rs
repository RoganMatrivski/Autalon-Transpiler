use wasm_bindgen::JsValue;

pub fn init() -> Result<(), JsValue> {
    set_up_errhook();
    tracing_wasm::set_as_global_default();

    Ok(())
}

#[cfg(feature = "console_error_panic_hook")]
fn set_up_errhook() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[cfg(not(feature = "console_error_panic_hook"))]
fn set_up_errhook() {}
