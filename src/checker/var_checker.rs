use std::collections::{hash_map::Entry, HashMap};

use crate::checker::{funcs::unwrap_inner, Checker};
use color_eyre::eyre::{bail, Report};
use pest::iterators::Pair;

use crate::autalonparser::Rule;

impl<'a> Default for Checker<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Checker<'a> {
    #[tracing::instrument]
    pub fn new() -> Checker<'a> {
        Checker {
            var_table: HashMap::new(),
        }
    }
}

impl<'a> Checker<'a> {
    #[tracing::instrument(skip_all)]
    pub fn check_var_declaration(&mut self, pair: Pair<'a, Rule>) -> Result<(), Report> {
        if pair.as_rule() != Rule::var_declaration {
            bail!("Pair is not a variable declaration");
        }

        let inner_pair = pair;
        let mut inner_pair = unwrap_inner(inner_pair)?.into_inner();

        let var_identifier = inner_pair.next().expect("Can't get variable identifier");
        let var_expr = inner_pair.next().expect("Can't get variable expression");

        let var_expression_type = self.get_expr_returntype(var_expr.into_inner())?;

        self.var_insert(var_identifier.as_str(), var_expression_type)?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub fn check_var_assignment(&mut self, pair: Pair<'a, Rule>) -> Result<(), Report> {
        if pair.as_rule() != Rule::var_assignment {
            bail!("Pair is not a variable declaration")
        }

        // Reassigning to scope into inner
        let mut inner_pair = pair.into_inner();

        let var_identifier = inner_pair.next().expect("Can't get variable identifier");
        let var_expr = inner_pair.next().expect("Can't get variable expression");

        let var_expr_type = self.get_expr_returntype(var_expr.into_inner())?;

        let current_var_type = self.var_lookup(var_identifier.as_str())?;

        if current_var_type != var_expr_type {
            bail!(
                "Expression type assigned to variable \"{}\" didn't match",
                var_identifier.as_str().to_string()
            )
        } else {
            Ok(())
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn var_insert(&mut self, name: &'a str, vartype: &'a str) -> Result<(), Report> {
        let entry = self.var_table.entry(name);
        match entry {
            Entry::Occupied(_) => bail!("Variable already exists!"),
            Entry::Vacant(_) => {
                self.var_table.insert(name, vartype);
                Ok(())
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn var_lookup(&mut self, name: &'a str) -> Result<&'a str, Report> {
        let entry = self.var_table.entry(name);
        if let Entry::Occupied(keyval) = entry {
            Ok(keyval.get())
        } else {
            bail!("Variable doesn't exist!")
        }
    }
}
