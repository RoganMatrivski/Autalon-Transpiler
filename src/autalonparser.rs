use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser_grammars/autalon.pest"]
pub struct AutalonParser;
