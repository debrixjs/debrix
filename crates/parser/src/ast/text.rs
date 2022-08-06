use crate::ast::*;

pub struct Text {
	pub location: Location,
	pub content: String,
}

impl IntoNode for Text {
	fn into_node(self) -> Node {
		Node::Text(self)
	}
}

pub struct TextBinding {
	pub location: Location,
	pub expression: javascript::Expression,
}

impl IntoNode for TextBinding {
	fn into_node(self) -> Node {
		Node::TextBinding(self)
	}
}
