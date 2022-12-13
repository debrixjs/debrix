use crate::ast::*;

#[derive(Debug)]
pub struct Text {
	pub start: usize,
	pub end: usize,
	pub content: String,
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

impl From<TextBinding> for Node {
	fn from(node: TextBinding) -> Node {
		Node::TextBinding(node)
	}
}
