pub mod ast;
mod error;
mod location;
mod parser;
mod scanner;

pub(crate) use {self::parser::*, self::scanner::*, std::fmt};

pub use self::{error::ParserError, parser::Parser};

#[cfg(test)]
mod tests;

pub fn parse_document(input: String) -> Result<ast::Document, ParserError> {
	Ok(ast::Document {
		children: parse(input)?,
	})
}

pub fn parse(input: String) -> Result<Vec<ast::Node>, ParserError> {
	let mut parser = Parser::new(input);
	let mut nodes = Vec::new();

	while let Some(node) = parser.next()? {
		nodes.push(node);
	}

	Ok(nodes)
}
