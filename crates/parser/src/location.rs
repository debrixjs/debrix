use crate::*;

#[derive(Clone)]
pub struct Position {
	pub line: usize,
	pub column: usize,
}

impl Position {
	pub fn new(line: usize, column: usize) -> Self {
		Self { line, column }
	}

	pub fn from_offset(offset: usize, content: &str) -> Self {
		let chars = content.chars().collect::<Vec<_>>();
		let mut line = 0;
		let mut column = 0;
		for i in 0..offset {
			if chars[i] == '\n' {
				line += 1;
				column = 0;
			} else {
				column += 1;
			}
		}
		Self::new(line, column)
	}

	pub fn offset(&self, content: &str) -> usize {
		let chars = content.chars().collect::<Vec<_>>();
		let mut offset = 0;
		let mut line = 0;
		let mut column = 0;
		while line < self.line || column < self.column {
			if chars[offset] == '\n' {
				line += 1;
				column = 0;
			} else {
				column += 1;
			}
			offset += 1;
		}
		offset
	}
}

impl Default for Position {
	fn default() -> Self {
		Self::new(0, 0)
	}
}

impl fmt::Debug for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.line, self.column)
	}
}

#[derive(Clone)]
pub struct Location {
	pub start: Position,
	pub end: Position,
}

impl Location {
	pub fn new(start: Position, end: Position) -> Self {
		Self { start, end }
	}

	pub fn from_offsets(start: usize, end: usize, content: &str) -> Self {
		Self::new(
			Position::from_offset(start, content),
			Position::from_offset(end, content),
		)
	}

	pub fn from_length(start: usize, length: usize, content: &str) -> Self {
		Self::from_offsets(start, start + length, content)
	}
}

impl Default for Location {
	fn default() -> Self {
		Self::new(Position::default(), Position::default())
	}
}

impl fmt::Debug for Location {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?} -> {:?}", self.start, self.end)
	}
}
