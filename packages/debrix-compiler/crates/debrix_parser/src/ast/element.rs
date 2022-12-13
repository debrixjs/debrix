use crate::ast::*;

#[derive(Debug)]
pub struct Element {
	pub start: usize,
	pub end: usize,
	pub tag_name: Identifier,
	pub end_tag: Option<Range>,
	pub attributes: Vec<Attribute>,
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
	Binding(BindingAttribute),
	Spread(SpreadAttribute),
	ShortBinding(ShortBindingAttribute)
}

#[derive(Debug)]
pub struct StaticAttribute {
	pub start: usize,
	pub end: usize,
	pub name: Identifier,
	pub value: Option<StringLiteral>,
}

#[derive(Debug)]
pub struct BindingAttribute {
	pub start: usize,
	pub end: usize,
	pub name: Identifier,
	pub value: javascript::Expression,
}

#[derive(Debug)]
pub struct SpreadAttribute {
	pub start: usize,
	pub end: usize,
	pub value: javascript::Expression,
}

#[derive(Debug)]
pub struct ShortBindingAttribute {
	pub start: usize,
	pub end: usize,
	pub name: javascript::IdentifierExpression,
}
