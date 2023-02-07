use eyre::{eyre, Report};

use crate::autalonparser::Rule;

pub fn check_type_arithmetic(lhs: &str, rhs: &str) -> Result<(), Report> {
    // TODO: Use oncecell for these kind of types maybe?
    let map = std::collections::HashMap::from([
        ("string", vec!["string"]),
        ("number", vec!["number"]),
        ("bool", vec!["bool"]),
    ]);

    match map.get(lhs) {
        None => Err(eyre!(
            "Cannot find type '{lhs}' for type arithmetic compatibility checking."
        )),
        Some(val) => match val.iter().position(|&x| x == rhs) {
            None => Err(eyre!("Type '{rhs}' is incompatible with '{lhs}'!")),
            Some(_) => Ok(()),
        },
    }
}

pub fn check_arithmetic_op(rhs: &str, optype: Rule) -> Result<(), Report> {
    // TODO: Use oncecell for these kind of types maybe?
    let map = std::collections::HashMap::from([
        ("string", vec![Rule::add_op]),
        (
            "number",
            vec![
                Rule::add_op,
                Rule::sub_op,
                Rule::div_op,
                Rule::mul_op,
                Rule::mod_op,
                Rule::pow_op,
            ],
        ),
    ]);

    match map.get(rhs) {
        None => Err(eyre!(
            "Cannot find type '{rhs}' for arithmetic operation compatibility checking."
        )),
        Some(val) => match val.iter().position(|x| x == &optype) {
            None => Err(eyre!("Type '{rhs}' is incompatible with '{optype:?}'!")),
            Some(_) => Ok(()),
        },
    }
}

pub fn check_comparation_op(rhs: &str, op_type: Rule) -> Result<(), Report> {
    // TODO: Use oncecell for these kind of types maybe?
    let map = std::collections::HashMap::from([
        ("string", vec![Rule::eq_op, Rule::ne_op]),
        ("bool", vec![Rule::eq_op, Rule::ne_op]),
        (
            "number",
            vec![
                Rule::eq_op,
                Rule::ne_op,
                Rule::lt_op,
                Rule::le_op,
                Rule::gt_op,
                Rule::ge_op,
            ],
        ),
    ]);

    match map.get(rhs) {
        None => Err(eyre!(
            "Cannot find type for type comparation compatibility checking."
        )),
        Some(val) => match val.iter().position(|x| x == &op_type) {
            None => Err(eyre!("Type '{rhs}' is incompatible with '{op_type:?}'!")),
            Some(_) => Ok(()),
        },
    }
}
