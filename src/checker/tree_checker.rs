use pest::iterators::Pair;

use crate::Rule;

use super::checker_funcs;

pub fn expr_tree_checker(
    lhs: Result<String, String>,
    op_pair: Pair<Rule>,
    rhs: Result<String, String>,
) -> Result<String, String> {
    // Unwrapping results, or rethrowing err if it's not Ok
    let lhs = match lhs {
        Ok(x) => x,
        Err(x) => return Err(x),
    };
    let rhs = match rhs {
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    // Arithmetic op compat check with rhs
    checker_funcs::check_arith_op_compat(&rhs, op_pair.as_rule())?;

    // LHS and RHS arithmetic op type compat check
    checker_funcs::check_type_arith_compat(&lhs, &rhs)?;

    // If all ok, return left hand side type
    Ok(lhs)
}

pub fn logic_comp_tree_checker(
    lhs: Result<String, String>,
    op_pair: Pair<Rule>,
    rhs: Result<String, String>,
) -> Result<String, String> {
    // Unwrapping results, or rethrowing err if it's not Ok
    let lhs = match lhs {
        Ok(x) => x,
        Err(x) => return Err(x),
    };
    let rhs = match rhs {
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    // Arithmetic op compat check with rhs
    checker_funcs::check_comp_op_compat(&rhs, op_pair.as_rule())?;

    // LHS and RHS arithmetic op type compat check
    checker_funcs::check_type_arith_compat(&lhs, &rhs)?;

    // Type match check
    checker_funcs::check_logic_comp_type(&lhs, &rhs)?;

    // If all ok, return left hand side type
    Ok(lhs)
}
