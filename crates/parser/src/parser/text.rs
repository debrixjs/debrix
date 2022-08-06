use crate::*;

impl Parser {
	pub fn parse_text(&mut self) -> Result<ast::Text, ParserError> {
		let start = self.iter.position();

		let mut content = String::new();

		loop {
			if let Some(ch) = self.iter.peek() {
				if ch == '<' || ch == '{' {
					break;
				}

				content.push(ch);
				self.iter.next();
			} else {
				break;
			}
		}

		let end = self.iter.position();

		Ok(ast::Text {
			location: Location::new(start, end),
			content,
		})
	}

	pub fn parse_text_binding(&mut self) -> Result<ast::TextBinding, ParserError> {
		let start = self.iter.position();
		self.expect('{')?;
		let expression = self.parse_javascript_expression(&['}'])?;
		self.expect('}')?;
		let end = self.iter.position();

		Ok(ast::TextBinding {
			location: Location::new(start, end),
			expression,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_text() {
		let mut parser = Parser::new("foo");
		let text = parser.parse_text().unwrap();

		assert_eq!(text.content, "foo");
	}

	#[test]
	fn test_parse_text_binding() {
		let mut parser = Parser::new("{foo}");
		let text_binding = parser.parse_text_binding().unwrap();

		match text_binding.expression {
			ast::javascript::Expression::Identifier(ident) => {
				assert_eq!(ident.name, "foo");
			}
			_ => panic!("Expected identifier"),
		}
	}
}
