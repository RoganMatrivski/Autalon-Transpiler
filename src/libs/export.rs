use eyre::Context;

use crate::{autalonparser, builtin_package_definition, checker, transpiler};

pub fn transpile_groovy(code: &str) -> Result<String, color_eyre::Report> {
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
            Err(err) => eyre::bail!(format!("{:?}", err)),
        }
    }

    transpiler::program_handler(transpiler::TranspilerOption::Groovy, &checked_pair)
}

pub fn get_fn_metadata() -> Result<String, color_eyre::Report> {
    use builtin_package_definition::{get_fn_metadata, BuiltinPkgFunctions, FunctionMetadata};
    use strum::IntoEnumIterator;

    let metadata_list: Vec<FunctionMetadata> = BuiltinPkgFunctions::iter()
        .map(|x| get_fn_metadata(&x))
        .collect();

    serde_json::to_string_pretty(&metadata_list).context("Failed to format metadata list")
}
