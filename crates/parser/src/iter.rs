use crate::*;

pub struct ChIter {
	content: String,
	chars: Vec<char>,
	pos: usize,
}

impl ChIter {
	pub fn new(content: String) -> ChIter {
		ChIter {
			content: content.clone(),
			chars: content.chars().collect(),
			pos: 0,
		}
	}

	pub fn position(&self) -> Position {
		Position::from_offset(self.pos, &self.content)
	}

	pub fn offset(&self) -> usize {
		self.pos
	}

	pub fn borrow_content(&self) -> &str {
		&self.content
	}

	pub fn next(&mut self) -> Option<char> {
		let ch = self.chars.get(self.pos).cloned();
		self.pos += 1;
		ch
	}

	pub fn back(&mut self) {
		self.back_n(1)
	}

	pub fn back_n(&mut self, n: usize) {
		self.pos -= n;
	}

	pub fn peek(&self) -> Option<char> {
		self.chars.get(self.pos).cloned()
	}

	pub fn peek_next(&self) -> Option<char> {
		self.chars.get(self.pos + 1).cloned()
	}

	pub fn peek_n(&self, n: usize) -> Option<char> {
		self.chars.get(self.pos + n).cloned()
	}

	pub fn skip_n(&mut self, n: usize) {
		self.pos += n;
	}
}
