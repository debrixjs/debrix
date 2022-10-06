#[derive(Debug)]
pub enum Literal {
	Number(NumberLiteral),
	String(StringLiteral),
	Boolean(BooleanLiteral),
	Null(NullLiteral),
}

#[derive(Debug)]
pub struct StringLiteral {
	pub start: usize,
	pub end: usize,
	pub value: String,
	pub quote: char,
}

#[derive(Debug)]
pub struct NumberLiteral {
	pub start: usize,
	pub end: usize,
	pub value: f64,
}

#[derive(Debug)]
pub struct BooleanLiteral {
	pub start: usize,
	pub end: usize,
	pub value: bool,
}

#[derive(Debug)]
pub struct NullLiteral {
	pub start: usize,
	pub end: usize,
}
