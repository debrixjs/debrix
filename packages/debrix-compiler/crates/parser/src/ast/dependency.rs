use crate::ast::*;

#[derive(Debug)]
pub struct DependencyStatement {
	pub start: usize,
	pub end: usize,
	pub default: Option<DependencyDefaultSpecifier>,
	pub named: Option<Nodes<DependencyNamedSpecifier>>,
	pub source: StringLiteral,
}

impl DependencyStatement {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<DependencyStatement> for Range {
	fn from(node: DependencyStatement) -> Self {
		node.range()
	}
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

impl DependencyDefaultSpecifier {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<DependencyDefaultSpecifier> for Range {
    fn from(node: DependencyDefaultSpecifier) -> Self {
        node.range()
    }
}

#[derive(Debug)]
pub struct DependencyNamedSpecifier {
	pub start: usize,
	pub end: usize,
	pub imported: Identifier,
	pub local: Option<Identifier>,
	pub usage: Identifier,
}

impl DependencyNamedSpecifier {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<DependencyNamedSpecifier> for Range {
    fn from(node: DependencyNamedSpecifier) -> Self {
        node.range()
    }
}
