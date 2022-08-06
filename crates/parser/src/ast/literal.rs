use crate::ast::*;

pub enum Literal {
	Number(NumberLiteral),
	String(StringLiteral),
	Boolean(BooleanLiteral),
	Null(NullLiteral),
}

pub struct StringLiteral {
	pub location: Location,
	pub value: String,
	pub quote: char,
}

pub struct NumberLiteral {
	pub location: Location,
	pub value: f64,
}

pub struct BooleanLiteral {
	pub location: Location,
	pub value: bool,
}

pub struct NullLiteral {
	pub location: Location,
}
