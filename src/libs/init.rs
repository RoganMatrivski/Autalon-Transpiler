use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use wasm_bindgen::JsValue;

pub fn wasm_init() -> Result<(), JsValue> {
    set_up_errhook();

    // tracing_wasm::set_as_global_default();

    let wasm_layer_cfg = tracing_wasm::WASMLayerConfigBuilder::new()
        .set_console_config(tracing_wasm::ConsoleConfig::ReportWithConsoleColor)
        .set_report_logs_in_timings(true)
        .set_max_level(tracing::Level::INFO)
        .build();

    let wasm_layer = tracing_wasm::WASMLayer::new(wasm_layer_cfg);

    let subscriber = tracing_subscriber::Registry::default()
        // any number of other subscriber layers may be added before or
        // after the `ErrorLayer`...
        .with(ErrorLayer::default())
        .with(wasm_layer);

    // set the subscriber as the default for the application
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|err| format!("Failed to setup tracing_subscriber. {err:?}"))?;

    color_eyre::config::HookBuilder::new()
        .theme(color_eyre::config::Theme::light())
        .add_default_filters()
        .display_env_section(false)
        .install()
        .map_err(|err| format!("Failed to setup color_eyre. {err:?}"))?;

    // color_eyre::install().map_err(|err| format!("Failed to setup color_eyre. {err:?}"))?;

    tracing::info!(
        "Autalon Transpiler initialized! Currently running version: {}",
        env!("CARGO_PKG_VERSION")
    );

    Ok(())
}

#[cfg(feature = "console_error_panic_hook")]
fn set_up_errhook() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[cfg(not(feature = "console_error_panic_hook"))]
fn set_up_errhook() {}
