use std::io::Read;

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

pub fn get_groovy_project_dir(
    code: &str,
    project_zip: &[u8],
    driver_jar_name: &str,
    driver_jar_bytes: &[u8],
) -> Result<Vec<u8>, String> {
    use std::io::Write;
    use zip::{write::FileOptions, ZipWriter};

    // Clones new file, read, and append new files
    let mut_project_zip = Vec::from(project_zip);
    let seekable_project_zip = std::io::Cursor::new(mut_project_zip);
    let mut zip = ZipWriter::new_append(seekable_project_zip)
        .map_err(|err| format!("Failed to read project zip! {err}"))?;

    zip.start_file(format!("Drivers/{driver_jar_name}"), FileOptions::default())
        .map_err(|err| format!("Failed to set Autalon driver file! {err}"))?;
    zip.write(driver_jar_bytes)
        .map_err(|err| format!("Failed to write Autalon driver! {err}"))?;

    let res = zip
        .finish()
        .map_err(|err| format!("Failed to finish zip file! {err}"))?;

    let res = res
        .bytes()
        .collect::<Result<Vec<u8>, std::io::Error>>()
        .map_err(|err| format!("Failed to get zip file bytes! {err}"))?;
    Ok(res)
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
    fn pass_returntype_comparation() -> color_eyre::eyre::Result<()> {
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
    fn pass_returntype_comparationlogic() -> color_eyre::eyre::Result<()> {
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
    fn pass_returntype_logic() -> color_eyre::eyre::Result<()> {
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
    fn pass_returntype_expr_pkgmember_noargs() -> color_eyre::eyre::Result<()> {
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
    fn pass_returntype_expr_pkgmember_args() -> color_eyre::eyre::Result<()> {
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
    fn pass_vardeclaration() -> color_eyre::eyre::Result<()> {
        let mut test = AutalonParser::parse(Rule::statement, "var test = \"asd\";")?;
        checker::statement_checker(test.next().unwrap())?;
        Ok(())
    }

    #[test]
    #[wasm_bindgen_test]
    fn pass_varassignment() -> color_eyre::eyre::Result<()> {
        let mut test =
            AutalonParser::parse(Rule::statement, "var test = \"asd\"; test = \"testassign\"")?;
        checker::statement_checker(test.next().unwrap())?;
        Ok(())
    }
}
