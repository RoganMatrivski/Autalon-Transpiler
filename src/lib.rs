extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use builtin_pkg_definition::BuiltinPkgFunctions;
use strum::IntoEnumIterator;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "parser/expr.pest"]
struct AutalonParser;

// mod builtin_pkg_list;
mod builtin_pkg_definition;
mod checker;
mod common_func;
mod transpiler;
// mod config;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn startup() {
    utils::set_panic_hook();
    log(format!(
        "Autalon Transpiler initialized! Currently running version: {}",
        env!("CARGO_PKG_VERSION")
    )
    .as_str());
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, autalon-transpiler-wasm!");
}

#[wasm_bindgen]
pub fn transpile_groovy(code: &str) -> String {
    log(format!("Attempting to transpile code below:\n\n{}", code).as_str());

    let parsed = AutalonParser::parse(Rule::program, code).expect("Failed to parse");

    let mut checked_pair: Vec<Pair<Rule>> = vec![];

    for pair in parsed {
        if pair.as_rule() == Rule::cfg_section {
            continue; // Skip cfg for now
        }

        if pair.as_rule() == Rule::EOI {
            break; // End of Input
        }

        match checker::statement_checker(pair.clone()) {
            Ok(_) => checked_pair.push(pair),
            Err(err) => panic!("ERROR! : {}", err),
        }
    }

    transpiler::program_handler(checked_pair).unwrap()
}

#[wasm_bindgen]
pub fn get_fn_metadata() -> String {
    let metadata_list: Vec<builtin_pkg_definition::FunctionMetadata> = BuiltinPkgFunctions::iter()
        .map(|x| builtin_pkg_definition::get_fn_metadata(&x))
        .collect();

    serde_json::to_string_pretty(&metadata_list).expect("Cannot stringify metadata_list")
}
