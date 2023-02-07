pub mod compat_check;
pub mod funcs;
pub mod tree_checker;
pub mod var_checker;

use std::collections::HashMap;

use crate::autalonparser::Rule;
use eyre::{bail, Report};
use pest::iterators::Pair;

pub struct Checker<'a> {
    var_table: HashMap<&'a str, &'a str>,
}

pub fn statement_checker(pair: Pair<Rule>) -> Result<(), Report> {
    let mut checker = Checker::new();

    for statement in pair.into_inner() {
        match statement.as_rule() {
            Rule::expr => {
                checker.get_expr_returntype(statement.into_inner())?;
            }
            Rule::var_declaration => {
                checker.check_var_declaration(statement)?;
            }
            Rule::var_assignment => {
                checker.check_var_assignment(statement)?;
            }
            Rule::escape_block => (),
            nonmatch => bail!("{nonmatch:?} doesn't match any return type!"),
        };
    }

    Ok(())
}
