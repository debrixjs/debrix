use crate::*;

#[derive(PartialEq, Clone)]
pub struct Position {
	pub line: usize,
	pub column: usize,
}

impl Position {
	pub fn new(line: usize, column: usize) -> Self {
		Self { line, column }
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

#[derive(PartialEq, Clone)]
pub struct Location {
	pub start: Position,
	pub end: Position,
}

impl Location {
	pub fn new(start: Position, end: Position) -> Self {
		Self { start, end }
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
