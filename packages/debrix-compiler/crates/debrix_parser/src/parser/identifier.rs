use crate::*;

pub fn is_identifier(char: &char) -> bool {
	char.is_alphanumeric() || char == &'_' || char == &'$'
}

impl Parser {
	pub fn parse_identifier(&mut self) -> Result<ast::Identifier, ParserError> {
		let start = self.scanner.cursor();
		let mut name = String::new();

		if let Some(char) = self.scanner.peek() {
			if !is_identifier(char) {
				return Err(self.expected(&["identifier"]));
			}

			name.push(*char);
		} else {
			return Err(self.unexpected());
		}

		while let Some(char) = self.scanner.next() {
			if is_identifier(char) {
				name.push(*char);
			} else {
				break;
			}
		}

		Ok(ast::Identifier {
			start,
			end: self.scanner.cursor(),
			name,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(input: &str) -> ast::Identifier {
		let mut parser = Parser::new(input.to_owned());
		parser.set_debug(true);
		let node = parser.parse_identifier().unwrap();
		assert!(parser.is_done());
		node
	}

	#[test]
	fn test_parse_identifier() {
		let ident = parse("abc");
		assert_eq!(ident.name, "abc");
	}
}
