use crate::ast::*;

#[derive(Debug)]
pub struct Element {
	pub start: usize,
	pub end: usize,
	pub tag_name: Identifier,
	pub end_tag: Option<Range>,
	pub attributes: Vec<Attribute>,
	pub bindings: Option<NodeCollection<Binding>>,
	pub children: Vec<Node>,
}

impl From<Element> for Node {
	fn from(node: Element) -> Node {
		Node::Element(node)
	}
}

#[derive(Debug)]
pub enum Attribute {
	Static(StaticAttribute),
	Binding(Binding),
}

#[derive(Debug)]
pub struct StaticAttribute {
	pub start: usize,
	pub end: usize,
	pub name: Identifier,
	pub value: Option<StringLiteral>,
}

#[derive(Debug)]
pub struct Binding {
	pub start: usize,
	pub end: usize,
	pub name: Identifier,
	pub value: javascript::Expression,
}
