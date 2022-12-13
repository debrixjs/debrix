use crate::ast::*;

#[derive(Debug)]
pub struct Comment {
	pub start: usize,
	pub end: usize,
	pub comment: String,
}

impl From<Comment> for Node {
	fn from(node: Comment) -> Node {
		Node::Comment(node)
	}
}
