pub mod ast;
mod error;
mod iter;
mod parser;
mod location;

use crate::ast::NodeCollection;
pub use crate::error::ParserError;

pub(crate) use crate::{
	ast::IntoNode,
	location::*,
	iter::*,
	parser::*,
};

pub(crate) use std::fmt;

pub fn parse(input: &str) -> Result<Vec<ast::Node>, ParserError> {
	let mut parser = Parser::new(input);
	let mut nodes = Vec::new();

	while let Some(node) = parser.next()? {
		nodes.push(node);
	}

	Ok(nodes)
}
