use crate::ast::*;

#[derive(Debug)]
pub struct Text {
	pub start: usize,
	pub end: usize,
	pub content: String,
}

impl Text {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<Text> for Range {
	fn from(node: Text) -> Self {
		node.range()
	}
}

impl From<Text> for Node {
	fn from(node: Text) -> Node {
		Node::Text(node)
	}
}

#[derive(Debug)]
pub struct TextBinding {
	pub start: usize,
	pub end: usize,
	pub expression: javascript::Expression,
}

impl TextBinding {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<TextBinding> for Range {
	fn from(node: TextBinding) -> Self {
		node.range()
	}
}

impl From<TextBinding> for Node {
	fn from(node: TextBinding) -> Node {
		Node::TextBinding(node)
	}
}
