use std::str::FromStr;

use crate::checker::{compat_check, funcs::unwrap_inner, Checker};
use color_eyre::eyre::{bail, eyre, Context, ContextCompat, Report};
use pest::iterators::{Pair, Pairs};

use crate::autalonparser::Rule;

impl<'a> Checker<'a> {
    #[tracing::instrument(skip(pair))]
    pub fn get_pair_returntype(&mut self, pair: Pair<'a, Rule>) -> Result<&'a str, Report> {
        match pair.as_rule() {
            Rule::string => Ok("string"),
            Rule::number => Ok("number"),
            Rule::bool => Ok("bool"),
            Rule::byoption_enum => Ok("byoption"),

            Rule::logic_op => self.get_logic_returntype(pair.into_inner()),
            Rule::comp_op => self.get_comp_returntype(pair.into_inner()),
            Rule::function_call => self.get_fnpair_returntype(pair),
            Rule::member_access => self.get_member_returntype(pair),

            Rule::array_access => todo!(),

            Rule::comparable => self.get_pair_returntype(unwrap_inner(pair)?),
            Rule::atomic_expression => self.get_pair_returntype(unwrap_inner(pair)?),
            Rule::basic_expr => self.get_pair_returntype(unwrap_inner(pair)?),

            nonmatch => Err(eyre!("{nonmatch:?} doesn't match any return type!")),
        }
    }

    #[tracing::instrument(skip(pairs))]
    pub fn get_expr_returntype(&mut self, pairs: Pairs<'a, Rule>) -> Result<&'a str, Report> {
        use pest::pratt_parser::{Assoc, Op, PrattParser};
        let parser = PrattParser::new()
            .op(Op::infix(Rule::add_op, Assoc::Left) | Op::infix(Rule::sub_op, Assoc::Left))
            .op(Op::infix(Rule::div_op, Assoc::Left) | Op::infix(Rule::mul_op, Assoc::Left));

        parser
            .map_primary(|x| self.get_pair_returntype(x))
            .map_infix(|lhs, op, rhs| {
                // Bubble up any errors from left/right side
                let lhs = lhs?;
                let rhs = rhs?;

                // Check if lhs type is the same as rhs
                if lhs != rhs {
                    return Err(eyre!("'{lhs}' type doesn't match with '{rhs}'"));
                }

                // Check if current operator is compatible with right hand side
                compat_check::check_arithmetic_op(rhs, op.as_rule())?;

                Ok(lhs)
            })
            .parse(pairs)
    }

    #[tracing::instrument(skip(comparable))]
    pub fn get_comparable_returntype(
        &mut self,
        comparable: Pair<'a, Rule>,
    ) -> Result<&'a str, Report> {
        match comparable.as_rule() {
            Rule::string => Ok("string"),
            Rule::number => Ok("number"),
            Rule::bool => Ok("bool"),
            Rule::byoption_enum => Ok("byoption"),

            nonmatch => Err(eyre!("'{nonmatch:?}' doesn't match any return type!")),
        }
    }

    #[tracing::instrument(skip(logic_pairs))]
    pub fn get_logic_returntype(
        &mut self,
        logic_pairs: Pairs<'a, Rule>,
    ) -> Result<&'a str, Report> {
        use pest::pratt_parser::{Assoc, Op, PrattParser};
        let parser = PrattParser::new()
            .op(Op::infix(Rule::and_op, Assoc::Left) | Op::infix(Rule::or_op, Assoc::Left));

        parser
            .map_primary(|primary| match primary.as_rule() {
                Rule::comp_op => self.get_comp_returntype(primary.into_inner()),

                _ => self.get_comparable_returntype(primary),
            })
            .map_infix(|lhs, _op, rhs| {
                // Bubble up any errors from left/right side
                let lhs = lhs?;
                let _rhs = rhs?;

                Ok(lhs)
            })
            .parse(logic_pairs)
    }

    #[tracing::instrument(skip(comp_op_pairs))]
    pub fn get_comp_returntype(
        &mut self,
        comp_op_pairs: Pairs<'a, Rule>,
    ) -> Result<&'a str, Report> {
        use pest::pratt_parser::{Assoc, Op, PrattParser};
        let parser = PrattParser::new()
            .op(Op::infix(Rule::ne_op, Assoc::Left) | Op::infix(Rule::eq_op, Assoc::Left))
            .op(Op::infix(Rule::ge_op, Assoc::Left) | Op::infix(Rule::le_op, Assoc::Left))
            .op(Op::infix(Rule::gt_op, Assoc::Left) | Op::infix(Rule::lt_op, Assoc::Left));

        parser
            .map_primary(|x| self.get_comparable_returntype(x))
            .map_infix(|lhs, op, rhs| {
                // Bubble up any errors from left/right side
                let lhs_type = lhs?;
                let rhs_type = rhs?;

                // Check if lhs type is the same as rhs
                if lhs_type != rhs_type {
                    return Err(eyre!("'{lhs_type}' type doesn't match with '{rhs_type}'"));
                }

                // Add comparation operation compatibility
                compat_check::check_comparation_op(rhs_type, op.as_rule())?;

                Ok(lhs_type)
            })
            .parse(comp_op_pairs)
    }

    fn get_fnpair_returntype(&mut self, pair: Pair<'a, Rule>) -> Result<&'a str, Report> {
        if pair.as_rule() != Rule::function_call {
            bail!(
                "Pair is not a function. Received input: {:?}",
                pair.as_rule()
            )
        }

        let mut inner_pair = pair.into_inner();
        let member = inner_pair
            .next()
            .context("Can't get member or function name!")?;
        let args = inner_pair.next();

        let fn_tokens = member.into_inner().into_iter().collect::<Vec<Pair<Rule>>>();
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

        let parsed_args = match args {
            None => vec![],
            Some(outer_pair) => outer_pair
                .into_inner() // Get inners
                .into_iter() // Into iterable
                .map(unwrap_inner) // Get args inner pair
                .collect::<Result<Vec<Pair<Rule>>, Report>>()? // Collect and get any errors
                .into_iter() // Into iterable again
                .map(|pair| pair.into_inner()) // Get all pairs inside iterated pair
                .map(|x| self.get_expr_returntype(x)) // Get iterated pair return type using all pairs inside
                .collect::<Result<Vec<&str>, Report>>()?, // Collect, and get any errors
        };

        let result = match pkg_name {
            None => bail!("Local package name is currently unsupported for now"),
            Some(pkg) => match val_name {
                Some(_) => bail!("Builtin value is currently unsupported for now"),
                None => self.get_pkgfn_returntype(fn_name.as_str(), pkg.as_str(), parsed_args)?,
            },
        };

        Ok(result)
    }

    #[tracing::instrument(skip(pair))]
    pub fn get_member_returntype(&mut self, pair: Pair<'a, Rule>) -> Result<&'a str, Report> {
        if pair.as_rule() != Rule::function_call {
            bail!(
                "Pair is not a function. Received input: {:?}",
                pair.as_rule()
            )
        }

        let inner_pair = pair.into_inner();
        let tokens = inner_pair.into_iter().collect::<Vec<Pair<Rule>>>();

        let (pkg_name, val_name) = match tokens.len() {
            2 => (Some(unwrap_inner(tokens[0].clone())?), &tokens[1]),
            1 => (None, &tokens[1]),
            _ => bail!(
                "Invalid function token length! Expected token count: 3; Received token count: {}",
                tokens.len()
            ),
        };

        let result = match pkg_name {
            Some(_) => bail!("Package name other than builtin ('#') is currently not supported"),
            None => self.var_lookup(val_name.as_str())?,
        };

        Ok(result)
    }

    #[tracing::instrument]
    pub fn get_pkgfn_returntype(
        &mut self,
        name: &'a str,
        pkg: &'a str,
        args: Vec<&'a str>,
    ) -> Result<&'a str, Report> {
        use crate::builtin_package_definition::{get_fn_metadata, BuiltinPkgFunctions};

        // TODO: Remove hardcoded package alias switching
        let pkg = if pkg == "#" { "builtin" } else { pkg };

        let function_metadata = match pkg {
            "builtin" => {
                let fn_enum = BuiltinPkgFunctions::from_str(name)
                    .context(format!("Failed to match enum {name}"))?;
                get_fn_metadata(&fn_enum)
            }

            _str => bail!("Package other than builtin is not implemented yet"),
        };

        if function_metadata.args.len() != args.len() {
            bail!(
            "Argument supplied for function \"{}\" didn't match. Supplied argument count \"{}\", expected argument count \"{}\"",
            name, args.len(), function_metadata.args.len())
        }

        let zipped_args = args
            .iter()
            .zip(function_metadata.args.iter().map(|x| x.arg_type))
            .collect::<Vec<(&&str, &str)>>();

        for (i, (arg, fn_metadata_arg)) in zipped_args.iter().enumerate() {
            if arg != &fn_metadata_arg {
                bail!(
                "Argument supplied for function \"{}\" didn't match. Supplied argument type for position {} is \"{}\", expected argument type for position {} is \"{}\"",
                name, i, arg, i, fn_metadata_arg)
            }
        }

        Ok(function_metadata.return_type)
    }
}
