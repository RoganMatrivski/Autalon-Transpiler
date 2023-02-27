pub mod autalonparser;
pub mod builtin_package_definition;
pub mod checker;
pub mod libs;
pub mod transpiler;

#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
pub use libs::export::*;
#[cfg(target_arch = "wasm32")]
pub use libs::wasm_export::*;
