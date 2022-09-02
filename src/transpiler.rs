use pest::iterators::Pair;

use crate::Rule;

pub mod katalon_prealpha;
pub mod package_func_map;

pub enum TranspilerOption {
    Groovy,
}

#[allow(unreachable_patterns)]
fn get_transpiler(option: TranspilerOption) -> impl Fn(Vec<Pair<Rule>>) -> Result<String, String> {
    match option {
        TranspilerOption::Groovy => katalon_prealpha::program_handler,
        _ => unimplemented!(),
    }
}

pub fn program_handler(pair: Vec<Pair<Rule>>) -> Result<String, String> {
    // TODO: Wrap these statements

    let transpiler = get_transpiler(TranspilerOption::Groovy);
    transpiler(pair)
}
