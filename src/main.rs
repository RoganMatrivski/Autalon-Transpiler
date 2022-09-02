extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use std::fs;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "parser/expr.pest"]
struct AutalonParser;
mod checker;
mod common_func;
mod transpiler;

fn main() {
    let unparsed_file = fs::read_to_string("testfile.atln").expect("cannot read file");
    let parsed = AutalonParser::parse(Rule::program, &unparsed_file).expect("Failed to parse");

    let mut checked_pair: Vec<Pair<Rule>> = vec![];

    for pair in parsed {
        if pair.as_rule() == Rule::cfg_section {
            continue; // Skip cfg for now
        }

        if pair.as_rule() == Rule::EOI {
            break; // End of Input
        }

        match checker::statement_checker(pair.clone()) {
            Ok(_) => checked_pair.push(pair),
            Err(err) => panic!("ERROR! : {}", err),
        }
    }

    println!("{}", transpiler::program_handler(checked_pair).unwrap());
}
