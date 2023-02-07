use wasm_bindgen::prelude::*;

pub mod autalonparser;
pub mod builtin_package_definition;
pub mod checker;
pub mod init;
pub mod transpiler;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn startup() -> Result<(), JsValue> {
    init::init()?;

    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, autalon-transpiler-wasm!");
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use crate::autalonparser::{AutalonParser, Rule};
    use crate::checker;
    use pest::Parser;

    #[test]
    #[wasm_bindgen_test]
    fn pass_returntype_comparation() -> eyre::Result<()> {
        let mut test = AutalonParser::parse(Rule::comp_op, "true == true")?;
        let mut checker = checker::Checker::new();

        println!(
            "{}",
            checker.get_comp_returntype(test.next().unwrap().into_inner())?
        );
        Ok(())
    }

    #[test]
    #[wasm_bindgen_test]
    fn pass_returntype_comparationlogic() -> eyre::Result<()> {
        let mut test = AutalonParser::parse(Rule::logic_op, "true && false == false && false")?;
        let mut checker = checker::Checker::new();

        println!(
            "{}",
            checker.get_logic_returntype(test.next().unwrap().into_inner())?
        );
        Ok(())
    }

    #[test]
    #[wasm_bindgen_test]
    fn pass_returntype_logic() -> eyre::Result<()> {
        let mut test = AutalonParser::parse(Rule::logic_op, "true && false")?;
        let mut checker = checker::Checker::new();

        println!(
            "{}",
            checker.get_logic_returntype(test.next().unwrap().into_inner())?
        );
        Ok(())
    }

    #[test]
    #[wasm_bindgen_test]
    fn pass_returntype_expr_pkgmember_noargs() -> eyre::Result<()> {
        let mut test = AutalonParser::parse(Rule::expr, "#:GetAndSwitchToAnyIFrame();")?;
        let mut checker = checker::Checker::new();

        println!(
            "{}",
            checker.get_expr_returntype(test.next().unwrap().into_inner())?
        );
        Ok(())
    }

    #[test]
    #[wasm_bindgen_test]
    fn pass_returntype_expr_pkgmember_args() -> eyre::Result<()> {
        let mut test = AutalonParser::parse(Rule::expr, "#:NavigateToUrl(\"google.com\");")?;
        let mut checker = checker::Checker::new();

        println!(
            "{}",
            checker.get_expr_returntype(test.next().unwrap().into_inner())?
        );
        Ok(())
    }

    #[test]
    #[wasm_bindgen_test]
    fn pass_vardeclaration() -> eyre::Result<()> {
        let mut test = AutalonParser::parse(Rule::statement, "var test = \"asd\";")?;
        checker::statement_checker(test.next().unwrap())?;
        Ok(())
    }

    #[test]
    #[wasm_bindgen_test]
    fn pass_varassignment() -> eyre::Result<()> {
        let mut test =
            AutalonParser::parse(Rule::statement, "var test = \"asd\"; test = \"testassign\"")?;
        checker::statement_checker(test.next().unwrap())?;
        Ok(())
    }
}
