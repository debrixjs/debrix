use crate::ast::*;

pub struct Comment {
	pub location: Location,
	pub comment: String,
}

impl IntoNode for Comment {
	fn into_node(self) -> Node {
		Node::Comment(self)
	}
}
