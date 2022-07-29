use pest::{Parser, iterators::Pairs};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct PestParser;

pub use Rule as R;

pub fn parse(input: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
	PestParser::parse(Rule::document, input)
}

#[cfg(test)]
mod tests;
