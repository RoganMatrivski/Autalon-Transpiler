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
