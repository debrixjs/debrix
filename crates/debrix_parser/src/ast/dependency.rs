use crate::ast::*;

#[derive(Debug)]
pub struct DependencyStatement {
	pub start: usize,
	pub end: usize,
	pub default: Option<DependencyDefaultSpecifier>,
	pub named: Option<NodeCollection<DependencyNamedSpecifier>>,
	pub source: StringLiteral,
}

impl From<DependencyStatement> for Node {
	fn from(node: DependencyStatement) -> Node {
		Node::DependencyStatement(node)
	}
}

#[derive(Debug)]
pub struct DependencyDefaultSpecifier {
	pub start: usize,
	pub end: usize,
	pub local: Option<Identifier>,
	pub usage: Identifier,
}

#[derive(Debug)]
pub struct DependencyNamedSpecifier {
	pub start: usize,
	pub end: usize,
	pub imported: Identifier,
	pub local: Option<Identifier>,
	pub usage: Identifier,
}
