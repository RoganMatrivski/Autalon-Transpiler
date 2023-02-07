use crate::autalonparser::Rule;
use eyre::Report;
use pest::iterators::Pair;

pub mod katalon_prealpha;

pub enum TranspilerOption {
    Groovy,
}

#[allow(unreachable_patterns)]
fn get_transpiler(option: TranspilerOption) -> impl Fn(&[Pair<Rule>]) -> Result<String, Report> {
    match option {
        TranspilerOption::Groovy => katalon_prealpha::program_handler,
        _ => unimplemented!(),
    }
}

pub fn program_handler(
    transpiler: TranspilerOption,
    pair: &[Pair<Rule>],
) -> Result<String, Report> {
    get_transpiler(transpiler)(pair)
}
