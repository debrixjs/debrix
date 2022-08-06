mod comment;
mod dependency;
mod element;
mod identifier;
mod javascript;
mod literal;
mod text;

use crate::*;

pub struct Parser {
	iter: ChIter,
}

impl Parser {
	pub fn new(input: &str) -> Self {
		Self {
			iter: ChIter::new(input.to_owned()),
		}
	}

	fn skip_whitespace(&mut self) {
		while let Some(ch) = self.iter.peek() {
			if ch.is_whitespace() {
				self.iter.next();
			} else {
				break;
			}
		}
	}

	fn expect(&mut self, ch: char) -> Result<(), ParserError> {
		if let Some(next) = self.iter.next() {
			if next == ch {
				Ok(())
			} else {
				Err(ParserError::expected(
					Location::from_length(
						self.iter.offset(),
						1,
						self.iter.borrow_content(),
					),
					&[&ch.to_string()],
				))
			}
		} else {
			Err(ParserError::eof(self.iter.position()))
		}
	}

	fn expect_str(&mut self, str: &str) -> Result<(), ParserError> {
		let start = self.iter.offset();

		for ch in str.chars() {
			if let Some(next) = self.iter.next() {
				if next != ch {
					return Err(ParserError::expected(
						Location::from_length(start, str.len(), self.iter.borrow_content()),
						&[str],
					));
				}
			} else {
				return Err(ParserError::eof(self.iter.position()));
			}
		}

		Ok(())
	}
	
	#[allow(dead_code)]
	fn try_ch(&mut self, ch: char) -> Result<bool, ParserError> {
		if let Some(next) = self.iter.peek() {
			if next == ch {
				self.iter.next();
				Ok(true)
			} else {
				Ok(false)
			}
		} else {
			Err(ParserError::eof(self.iter.position()))
		}
	}

	fn try_str(&mut self, str: &str) -> Result<bool, ParserError> {
		for ch in str.chars() {
			if let Some(next) = self.iter.next() {
				if next != ch {
					return Ok(false);
				}
			} else {
				return Err(ParserError::eof(self.iter.position()));
			}
		}

		Ok(true)
	}

	fn test(&mut self, ch: char) ->Result<bool, ParserError> {
		if let Some(next) = self.iter.peek() {
			Ok(next == ch)
		} else {
			return Err(ParserError::eof(self.iter.position()));
		}
	}

	fn test_str(&mut self, str: &str) -> Result<bool, ParserError> {
		for (i, ch) in str.chars().enumerate() {
			if let Some(next) = self.iter.next() {
				if next != ch {
					self.iter.back_n(i + 1);
					return Ok(false);
				}
			} else {
				return Err(ParserError::eof(self.iter.position()));
			}
		}

		self.iter.back_n(str.len());
		Ok(true)
	}

	pub fn next(&mut self) -> Result<Option<ast::Node>, ParserError> {
		if self.iter.peek_next().is_none() {
			return Ok(None);
		}

		if self.test_str("using")? {
			return Ok(Some(self.parse_dependency_statement()?.into_node()));
		}

		if self.test('<')? {
			return Ok(Some(self.parse_element()?.into_node()));
		}

		// invalid syntax
		Err(ParserError::unexpected(
			Location::from_length(self.iter.offset(), 1, &self.iter.borrow_content()),
			&[&self.iter.peek().unwrap().to_string()],
		))
	}
}
