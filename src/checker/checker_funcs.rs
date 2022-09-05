use pest::iterators::Pair;

use crate::{common_func::unwrap_inner, Rule};

use super::{
    static_var,
    tree_checker::{expr_tree_checker, logic_comp_tree_checker},
    var_state_checker::var_lookup,
};

pub fn get_pair_return_type(pair: Pair<Rule>) -> Result<String, String> {
    match pair.as_rule() {
        Rule::string => Ok("string".to_string()),
        Rule::number => Ok("number".to_string()),
        Rule::logic_op => get_logic_comp_return_type(pair),
        Rule::comp_op => get_logic_comp_return_type(pair),
        Rule::function_call => get_fn_pair_return_type(pair),
        Rule::member_access => get_member_return_type(pair),

        Rule::array_access => todo!(),

        Rule::comparable => get_pair_return_type(unwrap_inner(pair)),
        Rule::atomic_expression => get_pair_return_type(unwrap_inner(pair)),
        Rule::basic_expr => get_pair_return_type(unwrap_inner(pair)),
        _ => unreachable!("{:?} is not implemented yet!", pair.as_rule()),
    }
}

pub fn get_expr_return_type(pair: Pair<Rule>) -> Result<String, String> {
    static_var::ARITHMETIC_OP_CLIMBER.climb(
        pair.into_inner(),
        get_pair_return_type,
        expr_tree_checker,
    )
}

pub fn get_logic_comp_return_type(pair: Pair<Rule>) -> Result<String, String> {
    static_var::RELATION_OP_CLIMBER.climb(
        pair.clone().into_inner(),
        get_pair_return_type,
        logic_comp_tree_checker,
    )?;

    Ok("bool".to_string())
}

fn get_fn_pair_return_type(pair: Pair<Rule>) -> Result<String, String> {
    if pair.as_rule() != Rule::function_call {
        panic!("Pair is not a function")
    }

    let mut inner_pair = pair.into_inner();

    let member_acc = inner_pair.next().expect("Can't get member or fn name");
    let args = inner_pair.next(); // If none, no args

    let mut fn_tokens: Vec<Pair<Rule>> = vec![];

    for token in member_acc.into_inner() {
        fn_tokens.push(token);
    }

    let (pkg_name, val_name, fn_name) = match fn_tokens.len() {
        3 => (
            Some(unwrap_inner(fn_tokens.get(0).unwrap().to_owned())),
            Some(fn_tokens.get(1).unwrap()),
            fn_tokens.get(2).unwrap(),
        ),
        2 => {
            if fn_tokens[0].as_rule() == Rule::package {
                (
                    Some(unwrap_inner(fn_tokens.get(0).unwrap().to_owned())),
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

    let parsed_args = match args {
        None => vec![],
        Some(outer_pair) => {
            let mut arr = vec![];

            for pair in outer_pair.into_inner() {
                let arg_type = get_expr_return_type(pair)?;
                arr.push(arg_type);
            }

            arr
        }
    };

    let parsed_args = parsed_args.iter().map(|x| x.as_str()).collect();

    let res = match pkg_name {
        Some(pkg) => {
            // There's package name. check based on that

            match val_name {
                Some(_) => unimplemented!(), // No builtin value unfortunately for now, so skipping it.
                None => super::pkg_fn_checker::get_pkg_fn_return_type(
                    fn_name.as_str(),
                    pkg.as_str(),
                    parsed_args,
                ),
            }
        }
        None => unimplemented!(), // Fn is local, none of this one for now
    };

    // println!("{:?}, {:?}", fn_tokens, args);
    // println!("{:?}", args);
    // println!("{:?} {:?} {:?}", pkg_name, val_name, fn_name);

    res
    // Ok("()".to_string())
}

fn get_member_return_type(pair: Pair<Rule>) -> Result<String, String> {
    if pair.as_rule() != Rule::member_access {
        panic!("Pair is not a member access")
    }

    let inner_pair = pair.into_inner();

    let mut fn_tokens: Vec<Pair<Rule>> = vec![];

    for token in inner_pair {
        fn_tokens.push(token);
    }

    let (pkg_name, val_name) = match fn_tokens.len() {
        2 => (
            Some(unwrap_inner(fn_tokens.get(0).unwrap().to_owned())),
            fn_tokens.get(1).unwrap(),
        ),
        1 => (None, fn_tokens.get(0).unwrap()),
        _ => unreachable!(
            "Invalid function token length! Expected token count: 3; Received token count: {}",
            fn_tokens.len()
        ),
    };

    match pkg_name {
        Some(_) => {
            // Check for value on pkg_name. For now, it's unimplemented.
            unimplemented!();
        }
        None => {
            // Check var name to var symbol
            match var_lookup(val_name.to_string()) {
                Ok(vartype) => Ok(vartype),
                Err(err) => return Err(err),
            }
        }
    }
    // println!("{:?}", val_name);
}

pub fn check_type_arith_compat(lhs: &String, rhs: &String) -> Result<(), String> {
    // println!("{}", lhs);
    match static_var::TYPE_COMPARTION_COMPAT.get(lhs) {
        None => Err("Cannot find type for type arithmetic compatibility checking.".to_string()),
        Some(val) => match val.iter().position(|x| x == rhs) {
            None => Err("Type incompatible!".to_string()),
            Some(_) => Ok(()),
        },
    }
}

pub fn check_arith_op_compat(rhs: &String, optype: Rule) -> Result<(), String> {
    match static_var::ARITHMETIC_OP_COMPAT.get(rhs) {
        None => Err("Cannot find type for type arithmetic compatibility checking.".to_string()),
        Some(val) => match val.iter().position(|x| x == &optype) {
            None => Err("Type incompatible!".to_string()),
            Some(_) => Ok(()),
        },
    }
}

pub fn check_comp_op_compat(rhs: &String, optype: Rule) -> Result<(), String> {
    // println!("{}", rhs);
    match static_var::COMPARATION_OP_COMPAT.get(rhs) {
        None => Err("Cannot find type for type comparation compatibility checking.".to_string()),
        Some(val) => match val.iter().position(|x| x == &optype) {
            None => Err("Type incompatible!".to_string()),
            Some(_) => Ok(()),
        },
    }
}

pub fn check_logic_comp_type(lhs: &String, rhs: &String) -> Result<(), String> {
    if lhs == rhs {
        Ok(())
    } else {
        Err("Type mismatch!".to_string())
    }
}
