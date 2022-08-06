use crate::ast::*;

pub struct DependencyStatement {
	pub location: Location,
	pub default: Option<DependencyDefaultSpecifier>,
	pub named: Option<NodeCollection<DependencyNamedSpecifier>>,
	pub source: StringLiteral,
}

impl IntoNode for DependencyStatement {
	fn into_node(self) -> Node {
		Node::DependencyStatement(self)
	}
}

pub struct DependencyDefaultSpecifier {
	pub location: Location,
	pub local: Option<Identifier>,
	pub usage: Option<Identifier>,
}

pub struct DependencyNamedSpecifier {
	pub location: Location,
	pub imported: Option<Identifier>,
	pub local: Option<Identifier>,
	pub usage: Option<Identifier>,
}
