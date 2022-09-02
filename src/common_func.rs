use pest::iterators::Pair;

use crate::Rule;

pub fn unwrap_inner(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().next().expect("Cannot get inner pair")
}
