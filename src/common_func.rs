use pest::iterators::Pair;

use crate::Rule;

// &str to owned String macro
#[macro_export]
macro_rules! ostr {
    ($s:expr) => {
        $s.to_string()
    };
}

pub fn unwrap_inner(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().next().expect("Cannot get inner pair")
}
