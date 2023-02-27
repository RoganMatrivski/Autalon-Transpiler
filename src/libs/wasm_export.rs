use wasm_bindgen::prelude::*;

use crate::{autalonparser, builtin_package_definition, checker, transpiler};

use super::init;

#[wasm_bindgen(start)]
pub fn startup() -> Result<(), JsValue> {
    init::wasm_init()?;

    Ok(())
}

#[wasm_bindgen]
pub fn transpile_groovy(code: &str) -> Result<String, String> {
    use autalonparser::{AutalonParser, Rule};
    use pest::{iterators::Pair, Parser};

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
            Err(err) => return Err(format!("{:?}", err)),
        }
    }

    Ok(
        match transpiler::program_handler(transpiler::TranspilerOption::Groovy, &checked_pair) {
            Ok(res) => res,
            Err(err) => {
                tracing::error!(err = err.to_string(), "Failed transpiling script!");
                "".to_string()
            }
        },
    )
}

#[wasm_bindgen]
pub fn get_fn_metadata() -> Result<String, String> {
    use builtin_package_definition::{get_fn_metadata, BuiltinPkgFunctions, FunctionMetadata};
    use strum::IntoEnumIterator;

    let metadata_list: Vec<FunctionMetadata> = BuiltinPkgFunctions::iter()
        .map(|x| get_fn_metadata(&x))
        .collect();

    match serde_json::to_string_pretty(&metadata_list) {
        Ok(list) => Ok(list),
        Err(err) => Err(format!("{:?}", err)),
    }
}
