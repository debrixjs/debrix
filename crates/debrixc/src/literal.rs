use crate::parser::Rule;
use pest::iterators::Pair;

pub enum Literal {
	String(String),
	Number(f32),
	Boolean(bool),
}

impl Literal {
	pub fn parse(pair: &Pair<Rule>) -> Literal {
		let as_str = pair.as_str();

		match pair.as_rule() {
			Rule::string => Literal::String(as_str[1..as_str.len() - 1].to_owned()),
			Rule::number => Literal::Number(as_str.parse().unwrap()),
			Rule::boolean => Literal::Boolean(as_str.parse().unwrap()),
			_ => unreachable!(),
		}
	}

	pub fn as_str(&self) -> String {
		match self {
			Literal::String(string) => string.to_owned(),
			Literal::Number(number) => number.to_string(),
			Literal::Boolean(boolean) => boolean.to_string(),
		}
	}
}
