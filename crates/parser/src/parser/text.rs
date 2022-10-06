use crate::*;

impl Parser {
	pub fn parse_text(&mut self) -> Result<ast::Text, ParserError> {
		let start = self.scanner.cursor();
		let mut content = String::new();

		loop {
			if let Some(char) = self.scanner.peek() {
				if char == &'{' {
					break;
				}

				if char == &'<' || char == &'}' || char == &'#' {
					content = content.trim_end().to_owned();
					break;
				}

				content.push(char.to_owned());

				if char == &'\\' {
					if let Some(char) = self.scanner.next() {
						content.push(*char);
					} else {
						return Err(self.unexpected());
					}
				}
				
				self.scanner.next();
			} else {
				break;
			}
		}

		Ok(ast::Text {
			start,
			end: self.scanner.cursor(),
			content,
		})
	}

	pub fn parse_text_binding(&mut self) -> Result<ast::TextBinding, ParserError> {
		let start = self.scanner.cursor();

		if !self.scanner.take("{") {
			return Err(self.expected(&["{"]));
		}

		let expression = self.parse_javascript()?;

		if !self.scanner.take("}") {
			return Err(self.expected(&["}"]));
		}

		Ok(ast::TextBinding {
			start,
			end: self.scanner.cursor(),
			expression,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_text() {
		let mut parser = Parser::new("foo".to_owned());
		let text = parser.parse_text().unwrap();

		assert_eq!(text.content, "foo");
	}

	#[test]
	fn test_parse_text_binding() {
		let mut parser = Parser::new("{foo}".to_owned());
		let text_binding = parser.parse_text_binding().unwrap();

		match text_binding.expression {
			ast::javascript::Expression::Identifier(ident) => {
				assert_eq!(ident.name, "foo");
			}
			_ => panic!("Expected identifier"),
		}
	}
}
