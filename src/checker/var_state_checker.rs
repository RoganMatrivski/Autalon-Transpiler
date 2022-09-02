use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Mutex,
};

use pest::iterators::Pair;

use crate::{common_func::unwrap_inner, Rule};

use super::checker_funcs::get_expr_return_type;

lazy_static! {
    static ref VAR_TABLE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn check_var_declaration(pair: Pair<Rule>) -> Result<(), String> {
    if pair.as_rule() != Rule::var_declaration {
        panic!("Pair is not a variable declaration")
    }

    // Reassigning to scope into inner
    let inner_pair = pair;
    let mut inner_pair = unwrap_inner(inner_pair).into_inner();

    let var_identifier = inner_pair.next().expect("Can't get variable identifier");
    let var_expr = inner_pair.next().expect("Can't get variable expression");

    let var_expr_type = get_expr_return_type(var_expr)?;

    var_insert(
        var_identifier.as_str().to_string(),
        var_expr_type.to_string(),
    )
}

pub fn check_var_assignment(pair: Pair<Rule>) -> Result<(), String> {
    if pair.as_rule() != Rule::var_assignment {
        panic!("Pair is not a variable declaration")
    }

    // Reassigning to scope into inner
    let mut inner_pair = pair.into_inner();

    // PAIRDEBUG(unwrap_inner(inner_pair.clone()));

    let var_identifier = inner_pair.next().expect("Can't get variable identifier");
    let var_expr = inner_pair.next().expect("Can't get variable expression");

    let var_expr_type = get_expr_return_type(var_expr)?;

    let current_var_type = var_lookup(var_identifier.as_str().to_string())?;

    if current_var_type != var_expr_type {
        Err(format!(
            "Expression type assigned to variable \"{}\" didn't match",
            var_identifier.as_str().to_string()
        ))
    } else {
        Ok(())
    }
}

pub fn var_insert(name: String, vartype: String) -> Result<(), String> {
    let mut vartable = VAR_TABLE.lock().unwrap();
    let entry = vartable.entry(name.to_owned());
    match entry {
        Entry::Occupied(_) => Err("Variable already exists!".to_string()),
        Entry::Vacant(_) => {
            vartable.insert(name, vartype);
            Ok(())
        }
    }
}

pub fn var_lookup(name: String) -> Result<String, String> {
    let mut vartable = VAR_TABLE.lock().unwrap();
    let entry = vartable.entry(name);
    if let Entry::Occupied(keyval) = entry {
        Ok(keyval.get().to_string())
    } else {
        Err("Variable doesn't exist!".to_string())
    }
}

#[allow(dead_code)]
pub fn var_remove(name: String) -> Result<(), String> {
    let mut vartable = VAR_TABLE.lock().unwrap();
    let entry = vartable.entry(name);
    match entry {
        Entry::Occupied(keyval) => {
            keyval.remove();
            Ok(())
        }
        Entry::Vacant(_) => Err("Variable doesn't exist!".to_string()),
    }
}
