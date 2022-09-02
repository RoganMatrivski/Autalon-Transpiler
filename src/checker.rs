pub mod checker_funcs;
pub mod pkg_fn_checker;
pub mod static_var;
pub mod tree_checker;
pub mod var_state_checker;

use pest::iterators::Pair;

use crate::Rule;

pub fn statement_checker(pair: Pair<Rule>) -> Result<(), String> {
    for stmt in pair.into_inner() {
        match stmt.as_rule() {
            Rule::expr => {
                checker_funcs::get_expr_return_type(stmt)?;
            }
            Rule::var_declaration => var_state_checker::check_var_declaration(stmt)?,
            Rule::var_assignment => var_state_checker::check_var_assignment(stmt)?,
            Rule::escape_block => (),
            _ => unreachable!(),
        }
    }
    Ok(())
}
