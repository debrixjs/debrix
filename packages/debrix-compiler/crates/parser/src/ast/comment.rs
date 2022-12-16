use crate::ast::*;

#[derive(Debug)]
pub struct Comment {
	pub start: usize,
	pub end: usize,
	pub comment: String,
}

impl Comment {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<Comment> for Range {
	fn from(node: Comment) -> Self {
		node.range()
	}
}

impl From<Comment> for Node {
	fn from(node: Comment) -> Node {
		Node::Comment(node)
	}
}
