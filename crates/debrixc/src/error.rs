use std::fmt::Debug;

use pest::{error::ErrorVariant, RuleType};
use std::fmt;

#[derive(Debug)]
pub enum Error<'a, R: RuleType> {
	ParsingError(ParsingError<'a, R>),
	RenderingError(RenderingError<'a>),
}

pub enum InputLocation<'a> {
	Pos(pest::Position<'a>),
	Span(pest::Span<'a>),
}

impl<'a> Debug for InputLocation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InputLocation::Pos(pos) => {
				let line_col = pos.line_col();
				write!(f, "{}:{}: ", line_col.0, line_col.1)?;
			}
			InputLocation::Span(span) => {
				let start = span.start_pos().line_col();
				let end = span.end_pos().line_col();
				write!(f, "{}:{} -> {}:{}: ", start.0, start.1, end.0, end.1)?;
			}
		};

		Ok(())
	}
}

pub struct ParsingError<'a, R> {
	pub variant: ErrorVariant<R>,
	pub location: InputLocation<'a>,
}

impl<'a, R: RuleType> ParsingError<'a, R> {
	pub fn new_from_pos(variant: ErrorVariant<R>, pos: pest::Position<'a>) -> Self {
		Self {
			variant,
			location: InputLocation::Pos(pos),
		}
	}

	pub fn new_from_span(variant: ErrorVariant<R>, span: pest::Span<'a>) -> Self {
		Self {
			variant,
			location: InputLocation::Span(span),
		}
	}

	pub fn new_from_err(input: &'a str, err: pest::error::Error<R>) -> Self {
		Self {
			variant: err.variant,
			location: match err.location {
				pest::error::InputLocation::Pos(pos) => {
					InputLocation::Pos(pest::Position::new(input, pos).unwrap())
				}
				pest::error::InputLocation::Span(span) => {
					InputLocation::Span(pest::Span::new(input, span.0, span.1).unwrap())
				}
			},
		}
	}

	pub fn into_err(self) -> Error<'a, R> {
		Error::ParsingError(self)
	}
}

impl<R: RuleType> Debug for ParsingError<'_, R> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "{:?}", self.location)?;

		if let InputLocation::Pos(pos) = &self.location {
			writeln!(f, "{}", pos.line_of().replace("\r", "␍").replace("\n", "␊"))?;
			writeln!(f, "{}^", " ".repeat(pos.line_col().1 - 1))?;
		}

		match &self.variant {
			ErrorVariant::CustomError { message } => {
				writeln!(f, "{}", message.clone())?;
			}

			ErrorVariant::ParsingError {
				ref positives,
				ref negatives,
			} => {
				let enumerate = |rules: &[R]| -> String {
					match rules.len() {
						1 => format!("{:?}", &rules[0]),
						2 => format!("{:?} or {:?}", &rules[0], &rules[1]),
						l => {
							let separated = rules
								.iter()
								.take(l - 1)
								.map(|r| format!("{:?}", r))
								.collect::<Vec<_>>()
								.join(", ");
							format!("{}, or {:?}", separated, &rules[l - 1])
						}
					}
				};

				match (negatives.is_empty(), positives.is_empty()) {
					(false, false) => writeln!(
						f,
						"unexpected {}; expected {}",
						enumerate(negatives),
						enumerate(positives)
					)?,
					(false, true) => writeln!(f, "unexpected {}", enumerate(negatives))?,
					(true, false) => writeln!(f, "expected {}", enumerate(positives))?,
					(true, true) => writeln!(f, "unknown parsing error")?,
				};
			}
		}

		Ok(())
	}
}

pub struct RenderingError<'a> {
	pub message: String,
	pub location: Option<InputLocation<'a>>,
}

impl<'a> RenderingError<'a> {
	pub fn into_err<R>(self) -> Error<'a, R>
	where
		R: RuleType,
	{
		Error::RenderingError(self)
	}
}

impl<'a> Debug for RenderingError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "{:?}", self.location)?;
		writeln!(f, "{}", self.message)?;

		Ok(())
	}
}
