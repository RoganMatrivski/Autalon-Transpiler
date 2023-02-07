mod consts;
mod pkgdef;

use std::collections::HashMap;

use crate::{autalonparser::Rule, checker::funcs::unwrap_inner};
use eyre::{bail, ContextCompat, Report};
use pest::iterators::Pair;

pub fn program_handler(pair: &[Pair<Rule>]) -> Result<String, Report> {
    let res = pair
        .iter()
        .map(|x| unwrap_inner(x.clone()))
        .collect::<Result<Vec<Pair<Rule>>, Report>>()?
        .into_iter()
        .map(statement_handler)
        .collect::<Result<Vec<String>, Report>>()?
        .join("\n");

    Ok(consts::add_prepend(&res))
}

pub fn statement_handler(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(match pair.as_rule() {
        Rule::expr => expr_convert(pair)?,
        Rule::var_declaration => var_declaration_convert(pair)?,
        Rule::var_assignment => var_assignment_convert(pair)?,
        Rule::escape_block => "".to_string(),
        _ => unreachable!("{:?} is not implemented yet!", pair.as_rule()),
    } + ";")
}

fn pair_convert(pair: Pair<Rule>) -> Result<String, Report> {
    match pair.as_rule() {
        Rule::atomic_expression => pair_convert(unwrap_inner(pair)?),
        Rule::basic_expr => pair_convert(unwrap_inner(pair)?),

        Rule::comparable => comparable_convert(pair), // Ok("bool".to_string()),
        Rule::string => str_convert(pair),            // Ok("string".to_string()),
        Rule::number => number_convert(pair),         // Ok("number".to_string()),
        Rule::bool => bool_convert(pair),             // Ok("bool".to_string()),
        Rule::byoption_enum => byoption_convert(pair), // Ok("number".to_string()),

        Rule::logic_op => logicop_convert(pair), // Ok("bool".to_string()),
        Rule::comp_op => comp_op_convert(pair),  // Ok("bool".to_string()),

        Rule::function_call => fn_convert(pair), // get_fn_pair_return_type(pair),
        Rule::member_access => member_access_convert(pair), // get_member_return_type(pair),

        Rule::eq_op => compop_symbol_convert(pair),
        Rule::ne_op => compop_symbol_convert(pair),
        Rule::lt_op => compop_symbol_convert(pair),
        Rule::le_op => compop_symbol_convert(pair),
        Rule::gt_op => compop_symbol_convert(pair),
        Rule::ge_op => compop_symbol_convert(pair),

        Rule::and_op => logicop_symbol_convert(pair),
        Rule::or_op => logicop_symbol_convert(pair),

        Rule::add_op => arithop_symbol_convert(pair),
        Rule::sub_op => arithop_symbol_convert(pair),
        Rule::div_op => arithop_symbol_convert(pair),
        Rule::mul_op => arithop_symbol_convert(pair),
        Rule::mod_op => arithop_symbol_convert(pair),
        Rule::pow_op => arithop_symbol_convert(pair),

        _ => unimplemented!(),
    }
}

fn expr_convert(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(pair
        .into_inner()
        .into_iter()
        .map(pair_convert)
        .collect::<Result<Vec<String>, Report>>()?
        .join(" "))
}

fn fn_convert(pair: Pair<Rule>) -> Result<String, Report> {
    if pair.as_rule() != Rule::function_call {
        bail!("Pair is not a function")
    }

    let mut inner_pair = pair.into_inner();

    let member_acc = inner_pair.next().context("Can't get member or fn name")?;
    let args = inner_pair.next(); // If none, no args

    let fn_tokens = member_acc
        .into_inner()
        .into_iter()
        .collect::<Vec<Pair<Rule>>>();

    let (pkg_name, val_name, fn_name) = match fn_tokens.len() {
        3 => (
            Some(unwrap_inner(fn_tokens[0].clone())?),
            Some(&fn_tokens[1]),
            &fn_tokens[2],
        ),
        2 => {
            if fn_tokens[0].as_rule() == Rule::package {
                (
                    Some(unwrap_inner(fn_tokens[0].clone())?),
                    None,
                    &fn_tokens[1],
                )
            } else {
                (None, Some(&fn_tokens[0]), &fn_tokens[1])
            }
        }
        1 => (None, None, &fn_tokens[0]),
        _ => bail!(
            "Invalid function token length! Expected token count: 3; Received token count: {}",
            fn_tokens.len()
        ),
    };

    let converted_args = match args {
        None => vec![],
        Some(outer_pair) => outer_pair
            .into_inner()
            .into_iter()
            .map(expr_convert)
            .collect::<Result<Vec<String>, Report>>()?,
    };

    // Map arg values to [("{arg1}", argvalue), ("{arg2}", argvalue), ..., ("{argN}", argvalue)]
    // This is for the formatter that accepts {argN}
    let converted_args = converted_args
        .iter()
        .enumerate()
        .map(|(i, x)| (format!("arg{}", i + 1), x.to_owned()))
        .collect::<HashMap<String, String>>();

    let res = match pkg_name {
        Some(pkg) => {
            // There's package name. check based on that

            match val_name {
                Some(_) => unimplemented!(), // No builtin value unfortunately for now, so skipping it.
                None => self::pkgdef::get_default_fn_template(fn_name.as_str(), pkg.as_str())?,
            }
        }
        None => unimplemented!(), // Fn is local, none of this one for now
    };

    let formatted_res = if !converted_args.is_empty() {
        match strfmt::strfmt(res, &converted_args) {
            Ok(fmtstr) => fmtstr,
            Err(e) => panic!("{}", e),
        }
    } else {
        res.to_string()
    };

    Ok(formatted_res)
}

fn member_access_convert(pair: Pair<Rule>) -> Result<String, Report> {
    // For now, just return what's inside the cast
    let inner = unwrap_inner(pair)?;
    Ok(inner.as_str().to_string())
}

fn logicop_convert(pair: Pair<Rule>) -> Result<String, Report> {
    let mut inner_pair = pair.into_inner();

    let lhs = inner_pair
        .next()
        .context("Can't get left hand side from pair")?;
    let logic_op = inner_pair.next().context("Can't get operator from pair")?;
    let rhs = inner_pair
        .next()
        .context("Can't get right hand side from pair")?;

    let lhs_conv = pair_convert(lhs)?;
    let op_conv = pair_convert(logic_op)?;
    let rhs_conv = pair_convert(rhs)?;

    Ok(format!("{} {} {}", lhs_conv, op_conv, rhs_conv))
}

fn comp_op_convert(pair: Pair<Rule>) -> Result<String, Report> {
    let mut inner_pair = pair.into_inner();

    let lhs = inner_pair
        .next()
        .context("Can't get left hand side from pair")?;
    let comp_op = inner_pair.next().context("Can't get operator from pair")?;
    let rhs = inner_pair
        .next()
        .context("Can't get right hand side from pair")?;

    let lhs_conv = pair_convert(lhs)?;
    let op_conv = pair_convert(comp_op)?;
    let rhs_conv = pair_convert(rhs)?;

    Ok(format!("{} {} {}", lhs_conv, op_conv, rhs_conv))
}

fn var_declaration_convert(pair: Pair<Rule>) -> Result<String, Report> {
    let var_assignment = unwrap_inner(pair)?;

    let var_assignment_conv = var_assignment_convert(var_assignment)?;

    Ok(format!("def {}", var_assignment_conv))
}

fn var_assignment_convert(pair: Pair<Rule>) -> Result<String, Report> {
    let mut inner = pair.into_inner();

    let identifier = inner.next().context("Can't get variable identity")?;
    let expr = inner.next().context("Can't get variable expression")?;

    let identifier_str = identifier.as_str();
    let expr_parsed = expr_convert(expr)?;

    Ok(format!("{} = {}", identifier_str, expr_parsed))
}

fn comparable_convert(pair: Pair<Rule>) -> Result<String, Report> {
    let inner = unwrap_inner(pair)?;
    Ok(inner.as_str().to_string())
}

fn compop_symbol_convert(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(pair.as_str().to_string())
}

fn logicop_symbol_convert(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(pair.as_str().to_string())
}

fn arithop_symbol_convert(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(pair.as_str().to_string())
}

fn str_convert(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(pair.as_str().to_string())
}

fn number_convert(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(pair.as_str().to_string())
}

fn bool_convert(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(pair.as_str().to_string())
}

fn byoption_convert(pair: Pair<Rule>) -> Result<String, Report> {
    Ok(pair.as_str().to_string())
}
