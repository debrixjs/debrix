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

	fn skip_whitespace(&mut self) -> Result<(), ParserError> {
		'consume: while let Some(ch) = self.iter.peek() {
			if ch.is_whitespace() {
				self.iter.next();
				continue;
			}

			if ch == '/' {
				if let Some(ch) = self.iter.peek_next() {
					if ch == '/' {
						self.iter.skip_n(2);

						while let Some(ch) = self.iter.next() {
							if ch == '\n' {
								continue 'consume;
							}
						}
						
						// Reached EOF, which is also the end of the comment
						return Ok(());
					} else if ch == '*' {
						self.iter.skip_n(2);

						while let Some(ch) = self.iter.next() {
							if ch == '*' {
								if let Some(ch) = self.iter.peek() {
									if ch == '/' {
										self.iter.next();
										continue 'consume;
									}
								}
							}
						}

						return Err(ParserError::eof(self.iter.position()));
					}
				}
			}

			break;
		}
		
		return Ok(());
	}

	fn expect(&mut self, ch: char) -> Result<(), ParserError> {
		if let Some(next) = self.iter.next() {
			if next == ch {
				Ok(())
			} else {
				Err(ParserError::expected(
					Location::from_length(self.iter.offset(), 1, self.iter.borrow_content()),
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
	fn try_ch(&mut self, ch: char) -> bool {
		if let Some(next) = self.iter.peek() {
			if next == ch {
				self.iter.next();
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	fn try_str(&mut self, str: &str) -> bool {
		for ch in str.chars() {
			if let Some(next) = self.iter.next() {
				if next != ch {
					return false;
				}
			} else {
				return false;
			}
		}

		true
	}

	fn test(&mut self, ch: char) -> bool {
		if let Some(next) = self.iter.peek() {
			next == ch
		} else {
			false
		}
	}

	fn test_str(&mut self, str: &str) -> bool {
		for (i, ch) in str.chars().enumerate() {
			if let Some(next) = self.iter.next() {
				if next != ch {
					self.iter.back_n(i + 1);
					return false;
				}
			} else {
				return false;
			}
		}

		self.iter.back_n(str.len());
		true
	}

	pub fn next(&mut self) -> Result<Option<ast::Node>, ParserError> {
		if self.iter.peek_next().is_none() {
			return Ok(None);
		}

		if self.test_str("using") {
			return Ok(Some(self.parse_dependency_statement()?.into_node()));
		}

		if self.test('<') {
			return Ok(Some(self.parse_element()?.into_node()));
		}

		// invalid syntax
		Err(ParserError::unexpected(
			Location::from_length(self.iter.offset(), 1, &self.iter.borrow_content()),
			&[&self.iter.peek().unwrap().to_string()],
		))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// NOTE! This test module should only contain tests specific to the methods defined in this file.
	// Methods as well as their tests which are not common utillity should be specified in a seperate file.

	#[test]
	fn test_skip_whitespace() {
		let mut parser = Parser::new("foo bar");
		parser.iter.skip_n(3);
		parser.skip_whitespace().unwrap();
		assert_eq!(parser.iter.peek(), Some('b'));
	}

	#[test]
	fn test_skip_comment() {
		let mut parser = Parser::new("foo//bar\nbaz");
		parser.iter.skip_n(3);
		parser.skip_whitespace().unwrap();
		assert_eq!(parser.iter.peek(), Some('b'));
	}

	#[test]
	fn test_skip_multiline_comment() {
		let mut parser = Parser::new("foo/*\nbar\n*/baz");
		parser.iter.skip_n(3);
		parser.skip_whitespace().unwrap();
		assert_eq!(parser.iter.peek(), Some('b'));
	}
}
