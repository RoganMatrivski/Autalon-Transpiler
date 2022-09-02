use std::{collections::HashMap, sync::Mutex};

use pest::prec_climber::{self, Operator, PrecClimber};

use crate::Rule;

fn string_vecstring_tuple_mapper(val: (&str, Vec<&str>)) -> (String, Vec<String>) {
    (
        val.0.to_string(),
        val.1.iter().map(|x| x.to_string()).collect(),
    )
}

fn string_vecrule_tuple_mapper(val: (&str, Vec<Rule>)) -> (String, Vec<Rule>) {
    (val.0.to_string(), val.1)
}

lazy_static! {
    pub static ref PREC_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::add_op, prec_climber::Assoc::Left)
            | Operator::new(Rule::sub_op, prec_climber::Assoc::Left),
        Operator::new(Rule::div_op, prec_climber::Assoc::Left)
            | Operator::new(Rule::mul_op, prec_climber::Assoc::Left),
    ]);
    pub static ref ARITHMETIC_OP_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::add_op, prec_climber::Assoc::Left)
            | Operator::new(Rule::sub_op, prec_climber::Assoc::Left),
        Operator::new(Rule::div_op, prec_climber::Assoc::Left)
            | Operator::new(Rule::mul_op, prec_climber::Assoc::Left)
            | Operator::new(Rule::mod_op, prec_climber::Assoc::Left),
        Operator::new(Rule::pow_op, prec_climber::Assoc::Left),
    ]);
    pub static ref RELATION_OP_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::ne_op, prec_climber::Assoc::Left)
            | Operator::new(Rule::eq_op, prec_climber::Assoc::Left),
        Operator::new(Rule::ge_op, prec_climber::Assoc::Left)
            | Operator::new(Rule::le_op, prec_climber::Assoc::Left),
        Operator::new(Rule::gt_op, prec_climber::Assoc::Left)
            | Operator::new(Rule::lt_op, prec_climber::Assoc::Left)
    ]);
    pub static ref VAR_TABLE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    pub static ref TYPE_ARITHMETIC_COMPAT: HashMap<String, Vec<String>> = HashMap::from(
        [
            ("string", vec!["string", "number"]),
            ("number", vec!["string ", "number"])
        ]
        .map(string_vecstring_tuple_mapper)
    );
    pub static ref TYPE_COMPARTION_COMPAT: HashMap<String, Vec<String>> = HashMap::from(
        [
            ("string", vec!["string"]),
            ("number", vec!["number"]),
            ("bool", vec!["bool"])
        ]
        .map(string_vecstring_tuple_mapper)
    );
    pub static ref ARITHMETIC_OP_COMPAT: HashMap<String, Vec<Rule>> = HashMap::from(
        [
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
                ]
            ),
        ]
        .map(string_vecrule_tuple_mapper)
    );
    pub static ref COMPARATION_OP_COMPAT: HashMap<String, Vec<Rule>> = HashMap::from(
        [
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
                ]
            ),
        ]
        .map(string_vecrule_tuple_mapper)
    );
}
