use crate::*;

impl Parser {
	pub fn parse_string(&mut self) -> Result<ast::StringLiteral, ParserError> {
		let start = self.scanner.cursor();
		let mut value = String::new();

		if let Some(char) = self.scanner.peek().cloned() {
			if char == '\'' || char == '"' {
				let quote = char.to_owned();

				while let Some(char) = self.scanner.next() {
					if char == &quote {
						break;
					} else {
						value.push(char.to_owned());
					}
				}

				// skip the quote
				self.scanner.next();

				return Ok(ast::StringLiteral {
					start,
					end: self.scanner.cursor(),
					value,
					quote,
				});
			} else {
				return Err(self.expected(&["\"", "'"]));
			}
		}

		Err(self.unexpected())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn new_parser(input: &str) -> Parser {
		let mut parser = Parser::new(input.to_owned());
		parser.set_debug(true);
		parser
	}

	#[test]
	fn test_parse_string() {
		let mut parser = new_parser("'foo'");
		let string = parser.parse_string().unwrap();

		assert_eq!(string.value, "foo");
		assert_eq!(string.quote, '\'');
	}
}
