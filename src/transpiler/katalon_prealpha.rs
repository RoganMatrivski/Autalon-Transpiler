use std::collections::HashMap;

use pest::iterators::Pair;
use strfmt::strfmt;

use crate::{common_func::unwrap_inner, Rule};

use super::katalon_prealpha_pkg_def::get_default_fn_template;

const PREFIX_PROG: &str = r#"
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.Keys
import org.openqa.selenium.WebDriver
import org.openqa.selenium.WebElement
import org.openqa.selenium.By as By
import com.nawadata.nfunittestlibrary.Tools
import com.nawadata.nfunittestlibrary.Consts
import com.nawadata.nfunittestlibrary.Enums.*
import com.nawadata.nfunittestlibrary.Enums.ByOption
import com.nawadata.nfunittestlibrary.WebDriverExtended
import com.nawadata.nfunittestlibrary.WebElementExtended
import com.nawadata.nfunittestlibrary.ExtUIGetter
import com.nawadata.nfunittestlibrary.ReactMUIGetter

WebUI.openBrowser("")
WebUI.setViewPortSize(1280, 720)
WebUI.maximizeWindow()

def driver = DriverFactory.getWebDriver()
def driverExt = new WebDriverExtended(driver)
"#;

pub fn program_handler(pair: Vec<Pair<Rule>>) -> Result<String, String> {
    let res: Vec<String> = pair
        .iter()
        .map(|x| statement_handler(unwrap_inner(x.to_owned())).unwrap())
        .collect();

    Ok(PREFIX_PROG.to_string() + "\n" + &res.join("\n"))
}

pub fn statement_handler(pair: Pair<Rule>) -> Result<String, String> {
    let stmt = pair;

    PAIRDEBUG(stmt.clone());

    Ok(match stmt.as_rule() {
        Rule::expr => expr_convert(stmt)?,
        Rule::var_declaration => var_declaration_convert(stmt)?,
        Rule::var_assignment => var_assignment_convert(stmt)?,
        Rule::escape_block => "".to_string(),
        _ => unreachable!("{:?} is not implemented yet!", stmt.as_rule()),
    } + ";")
}

fn expr_convert(pairs: Pair<Rule>) -> Result<String, String> {
    let mut expr_arr = vec![];
    for expr in pairs.into_inner() {
        // PAIRDEBUG(stmt.clone());

        let res = pair_str_convert(expr)?;
        // println!("{}", res.clone());
        expr_arr.push(res);
    }

    let expr_arr_str = expr_arr.join(" ");

    Ok(expr_arr_str)
}

fn pair_str_convert(pairs: Pair<Rule>) -> Result<String, String> {
    // println!("Current rule: {:?}", pairs.as_rule());
    match pairs.as_rule() {
        Rule::atomic_expression => pair_str_convert(unwrap_inner(pairs)),
        Rule::basic_expr => pair_str_convert(unwrap_inner(pairs)),

        Rule::comparable => comparable_convert(pairs), // Ok("bool".to_string()),
        Rule::string => str_convert(pairs),            // Ok("string".to_string()),
        Rule::number => number_convert(pairs),         // Ok("number".to_string()),
        Rule::bool => bool_convert(pairs),             // Ok("bool".to_string()),
        Rule::byoption_enum => byoption_convert(pairs), // Ok("number".to_string()),

        Rule::logic_op => logic_op_convert(pairs), // Ok("bool".to_string()),
        Rule::comp_op => comp_op_convert(pairs),   // Ok("bool".to_string()),

        Rule::function_call => fn_convert(pairs), // get_fn_pair_return_type(pairs),
        Rule::member_access => member_access_convert(pairs), // get_member_return_type(pairs),

        Rule::eq_op => comp_op_symbol_convert(pairs),
        Rule::ne_op => comp_op_symbol_convert(pairs),
        Rule::lt_op => comp_op_symbol_convert(pairs),
        Rule::le_op => comp_op_symbol_convert(pairs),
        Rule::gt_op => comp_op_symbol_convert(pairs),
        Rule::ge_op => comp_op_symbol_convert(pairs),

        Rule::and_op => logic_op_symbol_convert(pairs),
        Rule::or_op => logic_op_symbol_convert(pairs),

        Rule::add_op => arith_op_symbol_convert(pairs),
        Rule::sub_op => arith_op_symbol_convert(pairs),
        Rule::div_op => arith_op_symbol_convert(pairs),
        Rule::mul_op => arith_op_symbol_convert(pairs),
        Rule::mod_op => arith_op_symbol_convert(pairs),
        Rule::pow_op => arith_op_symbol_convert(pairs),

        _ => unimplemented!(),
    }
}

fn comparable_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return what's inside the cast
    let inner = unwrap_inner(pairs);
    Ok(inner.as_str().to_string())
}

fn comp_op_symbol_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return as is.
    // If there's a case where there's a language not following common
    // comparation operator symbol, it'll be handled here.
    Ok(pairs.as_str().to_string())
}

fn logic_op_symbol_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return as is.
    // If there's a case where there's a language not following common
    // logic operator symbol, it'll be handled here.
    Ok(pairs.as_str().to_string())
}

fn arith_op_symbol_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return as is.
    // If there's a case where there's a language not following common
    // arithmetic operator symbol, it'll be handled here.
    Ok(pairs.as_str().to_string())
}

fn str_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return as is.
    // If there's a case where there's a language not following common
    // string grammar, it'll be handled here.
    Ok(pairs.as_str().to_string())
}

fn number_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return as is.
    // If there's a case where there's a language not following common
    // number, it'll be handled here.
    Ok(pairs.as_str().to_string())
}

fn bool_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return as is.
    // If there's a case where there's a language not following common
    // bool, it'll be handled here.
    Ok(pairs.as_str().to_string())
}

fn byoption_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return as is.
    // If there's a case where there's a language not following common
    // string grammar, it'll be handled here.
    Ok(pairs.as_str().to_string())
}

fn logic_op_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    let mut inner_pair = pairs.into_inner();

    let lhs = inner_pair
        .next()
        .expect("Can't get left hand side from pair");
    let logic_op = inner_pair.next().expect("Can't get operator from pair");
    let rhs = inner_pair
        .next()
        .expect("Can't get right hand side from pair");

    let lhs_conv = pair_str_convert(lhs)?;
    let op_conv = pair_str_convert(logic_op)?;
    let rhs_conv = pair_str_convert(rhs)?;

    Ok(format!("{} {} {}", lhs_conv, op_conv, rhs_conv))
}

fn comp_op_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    let mut inner_pair = pairs.into_inner();

    let lhs = inner_pair
        .next()
        .expect("Can't get left hand side from pair");
    let comp_op = inner_pair.next().expect("Can't get operator from pair");
    let rhs = inner_pair
        .next()
        .expect("Can't get right hand side from pair");

    let lhs_conv = pair_str_convert(lhs)?;
    let op_conv = pair_str_convert(comp_op)?;
    let rhs_conv = pair_str_convert(rhs)?;

    Ok(format!("{} {} {}", lhs_conv, op_conv, rhs_conv))
}

fn fn_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());
    if pairs.as_rule() != Rule::function_call {
        panic!("Pair is not a function")
    }

    let mut inner_pair = pairs.into_inner();

    let member_acc = inner_pair.next().expect("Can't get member or fn name");
    let args = inner_pair.next(); // If none, no args

    let mut fn_tokens: Vec<Pair<Rule>> = vec![];

    for token in member_acc.into_inner() {
        fn_tokens.push(token);
    }

    let (pkg_name, val_name, fn_name) = match fn_tokens.len() {
        3 => (
            Some(unwrap_inner(fn_tokens.get(0).unwrap().clone())),
            Some(fn_tokens.get(1).unwrap()),
            fn_tokens.get(2).unwrap(),
        ),
        2 => {
            if fn_tokens[0].as_rule() == Rule::package {
                (
                    Some(unwrap_inner(fn_tokens.get(0).unwrap().clone())),
                    None,
                    fn_tokens.get(1).unwrap(),
                )
            } else {
                (
                    None,
                    Some(fn_tokens.get(0).unwrap()),
                    fn_tokens.get(1).unwrap(),
                )
            }
        }
        1 => (None, None, fn_tokens.get(0).unwrap()),
        _ => unreachable!(
            "Invalid function token length! Expected token count: 3; Received token count: {}",
            fn_tokens.len()
        ),
    };

    let converted_args = match args {
        None => vec![],
        Some(outer_pair) => {
            let mut arr = vec![];

            for pair in outer_pair.into_inner() {
                let arg_converted = expr_convert(pair)?;
                arr.push(arg_converted);
            }

            arr
        }
    };

    // Map arg values to [("{arg1}", argvalue), ("{arg2}", argvalue), ..., ("{argN}", argvalue)]
    let converted_args: HashMap<String, &str> = converted_args
        .iter()
        .enumerate()
        .map(|(i, x)| (format!("arg{}", i + 1).to_string(), x.as_str()))
        .collect();

    let res = match pkg_name {
        Some(pkg) => {
            // There's package name. check based on that

            match val_name {
                Some(_) => unimplemented!(), // No builtin value unfortunately for now, so skipping it.
                None => get_default_fn_template(fn_name.as_str(), pkg.as_str()),
            }
        }
        None => unimplemented!(), // Fn is local, none of this one for now
    };
    // Ok(pairs.as_str().to_string());

    let formatted_res = if converted_args.len() > 0 {
        match strfmt(&res, &converted_args) {
            Ok(fmtstr) => fmtstr,
            Err(e) => panic!("{}", e),
        }
    } else {
        res
    };

    // let conv_res = format!("{}({})", res, formatted_args);

    Ok(formatted_res)
}

fn member_access_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    // For now, just return what's inside the cast
    let inner = unwrap_inner(pairs);
    Ok(inner.as_str().to_string())
}

fn var_declaration_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    let var_assignment = unwrap_inner(pairs);

    let var_assignment_conv = var_assignment_convert(var_assignment)?;

    Ok(format!("def {}", var_assignment_conv))
}

fn var_assignment_convert(pairs: Pair<Rule>) -> Result<String, String> {
    PAIRDEBUG(pairs.clone());

    let mut inner = pairs.into_inner();

    let identifier = inner.next().expect("Can't get variable identity");
    let expr = inner.next().expect("Can't get variable expression");

    let identifier_str = identifier.as_str();
    let expr_parsed = expr_convert(expr)?;

    Ok(format!("{} = {}", identifier_str, expr_parsed))
}

// TODO: Individualize this fn
#[allow(non_snake_case)]
fn PAIRDEBUG(pairs: Pair<Rule>) {
    println!("Rule:    {:?}", pairs.as_rule());
    println!("Span:    {:?}", pairs.as_span());
    println!("Text:    {}", pairs.as_str());
    println!();
}
