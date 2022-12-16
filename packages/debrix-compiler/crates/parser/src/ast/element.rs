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

impl Element {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<Element> for Range {
	fn from(node: Element) -> Self {
		node.range()
	}
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
	ShortBinding(ShortBindingAttribute),
}

impl Attribute {
	pub fn start(&self) -> usize {
		match self {
			Attribute::Static(range) => range.start,
			Attribute::Binding(range) => range.start,
			Attribute::Spread(range) => range.start,
			Attribute::ShortBinding(range) => range.start,
		}
	}

	pub fn end(&self) -> usize {
		match self {
			Attribute::Static(range) => range.end,
			Attribute::Binding(range) => range.end,
			Attribute::Spread(range) => range.end,
			Attribute::ShortBinding(range) => range.end,
		}
	}

	pub fn range(&self) -> Range {
		Range::new(self.start(), self.end())
	}
}

impl From<Attribute> for Range {
	fn from(node: Attribute) -> Self {
		node.range()
	}
}

#[derive(Debug)]
pub struct StaticAttribute {
	pub start: usize,
	pub end: usize,
	pub name: Identifier,
	pub value: Option<StringLiteral>,
}

impl StaticAttribute {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<StaticAttribute> for Range {
	fn from(node: StaticAttribute) -> Self {
		node.range()
	}
}

#[derive(Debug)]
pub struct BindingAttribute {
	pub start: usize,
	pub end: usize,
	pub name: Identifier,
	pub value: javascript::Expression,
}

impl BindingAttribute {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<BindingAttribute> for Range {
	fn from(node: BindingAttribute) -> Self {
		node.range()
	}
}

#[derive(Debug)]
pub struct SpreadAttribute {
	pub start: usize,
	pub end: usize,
	pub value: javascript::Expression,
}

impl SpreadAttribute {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<SpreadAttribute> for Range {
	fn from(node: SpreadAttribute) -> Self {
		node.range()
	}
}

#[derive(Debug)]
pub struct ShortBindingAttribute {
	pub start: usize,
	pub end: usize,
	pub name: javascript::IdentifierExpression,
}

impl ShortBindingAttribute {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<ShortBindingAttribute> for Range {
	fn from(node: ShortBindingAttribute) -> Self {
		node.range()
	}
}
