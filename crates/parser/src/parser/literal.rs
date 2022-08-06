use crate::*;

impl Parser {
	pub fn parse_string(&mut self) -> Result<ast::StringLiteral, ParserError> {
		let start = self.iter.position();
		let mut value = String::new();

		if let Some(ch) = self.iter.next() {
			if ch == '\'' || ch == '"' {
				let quote = ch;

				while let Some(ch) = self.iter.next() {
					if ch == quote {
						break;
					} else {
						value.push(ch);
					}
				}

				let end = self.iter.position();

				return Ok(ast::StringLiteral {
					value,
					quote,
					location: Location::new(start, end),
				});
			} else {
				// expected string
			}
		}

		// unexpected eof
		Err(ParserError::eof(self.iter.position()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_string() {
		let mut parser = Parser::new("'foo'");
		let string = parser.parse_string().unwrap();

		assert_eq!(string.value, "foo");
		assert_eq!(string.quote, '\'');
	}
}
