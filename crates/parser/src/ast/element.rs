use crate::ast::*;

pub struct Element {
	pub location: Location,
	pub tag: Identifier,
	pub self_closing: bool,
	pub attributes: Vec<Attribute>,
	pub bindings: Option<NodeCollection<Binding>>,
	pub children: Vec<Node>,
}

impl IntoNode for Element {
	fn into_node(self) -> Node {
		Node::Element(self)
	}
}

pub enum Attribute {
	Static(StaticAttribute),
	Binding(Binding),
}

pub struct StaticAttribute {
	pub location: Location,
	pub name: Identifier,
	pub value: Option<StringLiteral>,
}

pub struct Binding {
	pub location: Location,
	pub name: Identifier,
	pub value: javascript::Expression,
}
