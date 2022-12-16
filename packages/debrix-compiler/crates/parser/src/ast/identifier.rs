use crate::ast::*;

#[derive(Debug)]
pub struct Identifier {
	pub start: usize,
	pub end: usize,
	pub name: String,
}

impl Identifier {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<Identifier> for Range {
	fn from(node: Identifier) -> Self {
		node.range()
	}
}
