use color_eyre::eyre::{ContextCompat, Report};
use pest::iterators::Pair;

use crate::autalonparser::Rule;

#[tracing::instrument(skip_all)]
pub fn unwrap_inner(pair: Pair<Rule>) -> Result<Pair<Rule>, Report> {
    pair.into_inner().next().context("Failed to get inner pair")
}
