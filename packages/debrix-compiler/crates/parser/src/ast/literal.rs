use crate::ast::*;

#[derive(Debug)]
pub enum Literal {
	Number(NumberLiteral),
	String(StringLiteral),
	Boolean(BooleanLiteral),
	Null(NullLiteral),
}

impl Literal {
	pub fn start(&self) -> usize {
		match self {
			Literal::Number(literal) => literal.start,
			Literal::String(literal) => literal.start,
			Literal::Boolean(literal) => literal.start,
			Literal::Null(literal) => literal.start,
		}
	}

	pub fn end(&self) -> usize {
		match self {
			Literal::Number(literal) => literal.end,
			Literal::String(literal) => literal.end,
			Literal::Boolean(literal) => literal.end,
			Literal::Null(literal) => literal.end,
		}
	}

	pub fn range(&self) -> Range {
		Range::new(self.start(), self.end())
	}
}

impl From<Literal> for Range {
	fn from(node: Literal) -> Self {
		node.range()
	}
}

#[derive(Debug)]
pub struct StringLiteral {
	pub start: usize,
	pub end: usize,
	pub value: String,
	pub quote: char,
}

impl StringLiteral {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<StringLiteral> for Range {
	fn from(node: StringLiteral) -> Self {
		node.range()
	}
}

#[derive(Debug)]
pub struct NumberLiteral {
	pub start: usize,
	pub end: usize,
	pub value: f64,
}

impl NumberLiteral {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<NumberLiteral> for Range {
	fn from(node: NumberLiteral) -> Self {
		node.range()
	}
}

#[derive(Debug)]
pub struct BooleanLiteral {
	pub start: usize,
	pub end: usize,
	pub value: bool,
}

impl BooleanLiteral {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<BooleanLiteral> for Range {
	fn from(node: BooleanLiteral) -> Self {
		node.range()
	}
}

#[derive(Debug)]
pub struct NullLiteral {
	pub start: usize,
	pub end: usize,
}

impl NullLiteral {
	pub fn range(&self) -> Range {
		Range::new(self.start, self.end)
	}
}

impl From<NullLiteral> for Range {
	fn from(node: NullLiteral) -> Self {
		node.range()
	}
}
